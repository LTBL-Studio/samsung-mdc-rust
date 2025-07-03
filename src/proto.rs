//! Structures and methods to create and parse packets

use thiserror::Error;

/// A packet sent over MDC connection
/// Its carries commands and responses from screen
#[derive(Debug, PartialEq)]
pub struct Packet {
    /// Command id to perform (see [crate::commands] constants for a list of commands)
    pub command: u8,
    /// Display id to send command to (or [crate::commands::DISPLAY_BROADCAST] for a broadcast)
    pub display_id: u8,
    /// Data and arguments associated to this command
    pub data: Vec<u8>
}

impl Packet {
    /// Create a new packet with provided data
    pub fn new(command: u8, display_id: u8, data: Vec<u8>) -> Self {
        Self { command, display_id, data }
    }

    /// Compute packet's checksum
    pub fn checksum(&self) -> u8 {
        (
            self.command as i32 +
            self.display_id as i32 +
            self.data.len() as i32 +
            self.data.iter().map(|it| *it as i32).sum::<i32>()
        ) as u8
    }

    /// Convert this packet into bytes ready to be sent
    pub fn into_bytes(mut self) -> Vec<u8> {
        let checksum = self.checksum();
        let mut bytes = vec![
            0xAA,
            self.command,
            self.display_id,
            self.data.len() as u8
        ];
        bytes.append(&mut self.data);
        bytes.push(checksum);
        bytes
    }

    /// Parse packet from buffer, removing bytes associated to parsed packet from buffer.
    /// 
    /// Returns a packet and the number of bytes removed from buffer.
    /// In cas of error, buffer is not modified.
    pub fn from_bytes(input: &mut Vec<u8>) -> Result<(Self, usize), Error> {
        let Some(header) = input.first() else {
            return Err(Error::IncompleteInput)
        };

        if *header != 0xAA {
            return Err(Error::InvalidHeader);
        };

        let Some(command) = input.get(1).cloned() else {
            return Err(Error::IncompleteInput);
        };
        
        let Some(display_id) = input.get(2).cloned() else {
            return Err(Error::IncompleteInput);
        };
        
        let Some(data_length) = input.get(3).map(|it| *it as usize) else {
            return Err(Error::IncompleteInput);
        };

        let Some(given_checksum) = input.get(4+data_length).cloned() else {
            return Err(Error::IncompleteInput);
        };

        let checksum = (command as i32 + display_id as i32 + data_length as i32 + input[4..4+data_length].iter().map(|it| *it as i32).sum::<i32>()) as u8;

        if checksum != given_checksum {
            return Err(Error::InvalidChecksum)
        }

        if input.len() <= 4+data_length {
            return Err(Error::IncompleteInput)
        }

        let data = input.drain(..4+data_length+1).skip(4).take(data_length).collect::<Vec<_>>();

        let bytes_red = 4+data_length+1;

        Ok((Self {
            command,
            display_id,
            data
        }, bytes_red))
    }
}

/// Error that can occur during packet parsing
#[derive(Debug, Error)]
pub enum Error {
    /// Packet do not start with 0xAA header
    #[error("Invalid header: every packet should start with 0xAA")]
    InvalidHeader,
    /// Input buffer was incomplete and do not contains a full packet (it can means that you should request more bytes)
    #[error("Incomplete input")]
    IncompleteInput,
    /// Checksum received is not valid, that can means a corrupted packet
    #[error("Invalid Checksum")]
    InvalidChecksum
}

mod test {
    use super::Packet;

    #[test]
    pub fn should_compute_valid_checksum(){
        assert_eq!(Packet {
            command: 0x11,
            display_id: 0xFE,
            data: vec![1]
        }.checksum(), 0x11);

        assert_eq!(Packet {
            command: 0xB9,
            display_id: 0x00,
            data: vec![0x00]
        }.checksum(), 0xBA);
    }

    #[test]
    pub fn should_create_valid_packet_bytes(){
        assert_eq!(Packet {
            command: 0x4A,
            display_id: 0x00,
            data: vec![0x00]
        }.into_bytes(), vec![0xAA, 0x4A, 0x00, 0x01, 0x00, 0x4B]);
    }

    #[test]
    pub fn should_parse_bytes(){
        assert_eq!(Packet::from_bytes(&mut vec![0xAA, 0x4A, 0x00, 0x01, 0x00, 0x4B]).unwrap().0, Packet {
            command: 0x4A,
            display_id: 0x00,
            data: vec![0x00]
        });
    }

    #[test]
    pub fn should_parse_partial_bytes(){
        let mut input = vec![0xAA, 0x4A, 0x00, 0x01, 0x00, 0x4B, 0xAA, 0xFF];
        assert_eq!(Packet::from_bytes(&mut input).unwrap(), (Packet {
            command: 0x4A,
            display_id: 0x00,
            data: vec![0x00]
        }, 6));

        assert_eq!(input, vec![0xAA, 0xFF])
    }
}