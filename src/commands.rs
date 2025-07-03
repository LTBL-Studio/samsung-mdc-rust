//! Set of commands IDs

///Special Display id to send command to all displays
pub const DISPLAY_BROADCAST: u8 = 0xFE;

/// Acknowledge or Not Acknowledge response
pub const ACK_NACK:u8 = 0xFF;

/// Control power state of display
pub const POWER_CONTROL:u8 = 0x11;

/// Control panel On/Off
pub const PANEL_ON_OFF:u8 = 0xF9;