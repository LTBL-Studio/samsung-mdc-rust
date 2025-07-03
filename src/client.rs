//! Communicate with MDC screen

use std::{io::{Read, Write}, net::{SocketAddr, TcpStream}};

use crate::{commands, proto::{self, Packet}, DISPLAY_BROADCAST};

const INIT_BUFFER_SIZE: usize = 1024;

/// A trait representing a valid MDC stream to communicate on
pub trait MDCStream: Read + Write {}
impl<T: Read + Write> MDCStream for T {}

/// A MDC session where we can send and receive packets
pub struct MDCSession<S: MDCStream> {
    stream: S,
    buffer: Vec<u8>
}

impl MDCSession<TcpStream> {
    /// Initiate a new session over TCP
    pub fn new_from_tcp(addr: SocketAddr) -> Result<Self, crate::Error> {
        let connection = TcpStream::connect(addr)?;
        Self::new_from_stream(connection)
    }
}

impl<S: MDCStream> MDCSession<S> {
    /// Initiate a new connection from arbitrary stream
    pub fn new_from_stream(stream: S) -> Result<Self, crate::Error> {
        let new_self = Self {
            stream,
            buffer: Vec::with_capacity(INIT_BUFFER_SIZE)
        };
        Ok(new_self)
    }

    /// Send commands to a display ID
    pub fn display(&mut self, display_id: u8) -> DisplayCommandBuilder<'_, S> {
        DisplayCommandBuilder { session: self, display_id }
    }

    /// Send commands to all displays available in this session
    pub fn all_displays(&mut self) -> BroadcastCommandBuilder<'_, S> {
        BroadcastCommandBuilder { session: self }
    }

    /// Low level method to receive next packet
    pub fn recv_packet(&mut self) -> Result<Packet, crate::Error> {
        let mut buffer = [0_u8; INIT_BUFFER_SIZE];
        loop {
            match Packet::from_bytes(&mut self.buffer) {
                Ok((p, _)) => return Ok(p),
                Err(proto::Error::IncompleteInput) => {},
                Err(e) => {
                    self.buffer.clear();
                    return Err(crate::Error::InvalidPacket(e))
                }
            }

            let byte_red = self.stream.read(&mut buffer)?;
            if byte_red == 0 {
                return Err(crate::Error::UnexpectedEndOfStream)
            }
            self.buffer.extend_from_slice(&buffer[..byte_red]);
        }
    }

    /// Low level method to send a packet
    pub fn send_packet(&mut self, packet: impl Into<Packet>) -> Result<(), crate::Error> {
        let p: Packet = packet.into();
        self.stream.write_all(&p.into_bytes())?;
        Ok(())
    }

    /// Low level method to send a packet and then wait for a ACK message
    pub fn send_packet_ack(&mut self, packet: impl Into<Packet>) -> Result<Packet, crate::Error> {
        self.send_packet(packet)?;
        let response = self.recv_packet()?;

        if response.command != commands::ACK_NACK {
            return Err(crate::Error::UnexpectedResponse(response));
        }

        if response.data.first().is_none_or(|it| *it != b'A') {
            return Err(crate::Error::Nack(response));
        }

        Ok(response)
    }
}

/// A high level controller that can send screen commands and receive screen informations
pub trait DisplayControl {
    /// Set light panel on
    fn set_panel_on(&mut self) -> Result<(), crate::Error>;

    /// Set light panel off and blank screen
    fn set_panel_off(&mut self) -> Result<(), crate::Error>;

    /// Set screen power on
    fn set_power_on(&mut self) -> Result<(), crate::Error>;

    /// Set screen power off
    fn set_power_off(&mut self) -> Result<(), crate::Error>;
}

/// Send and receive commands for a specific display ID
pub struct DisplayCommandBuilder<'a, S: MDCStream> {
    session: &'a mut MDCSession<S>,
    display_id: u8
}

impl<S: MDCStream> DisplayControl for DisplayCommandBuilder<'_, S> {
    fn set_panel_off(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet_ack(Packet::new(commands::PANEL_ON_OFF, self.display_id, vec![1]))?;
        Ok(())
    }

    fn set_panel_on(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet_ack(Packet::new(commands::PANEL_ON_OFF, self.display_id, vec![0]))?;
        Ok(())
    }

    fn set_power_off(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet_ack(Packet::new(commands::POWER_CONTROL, self.display_id, vec![0]))?;
        Ok(())
    }

    fn set_power_on(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet_ack(Packet::new(commands::POWER_CONTROL, self.display_id, vec![1]))?;
        Ok(())
    }
}

/// Send and receive commands to all connected displays
pub struct BroadcastCommandBuilder<'a, S: MDCStream> {
    session: &'a mut MDCSession<S>
}

impl<S: MDCStream> DisplayControl for BroadcastCommandBuilder<'_, S> {
    fn set_panel_off(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet(Packet::new(commands::PANEL_ON_OFF, DISPLAY_BROADCAST, vec![1]))?;
        Ok(())
    }

    fn set_panel_on(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet(Packet::new(commands::PANEL_ON_OFF, DISPLAY_BROADCAST, vec![0]))?;
        Ok(())
    }

    fn set_power_off(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet(Packet::new(commands::POWER_CONTROL, DISPLAY_BROADCAST, vec![0]))?;
        Ok(())
    }

    fn set_power_on(&mut self) -> Result<(), crate::Error> {
        self.session.send_packet(Packet::new(commands::POWER_CONTROL, DISPLAY_BROADCAST, vec![1]))?;
        Ok(())
    }
}