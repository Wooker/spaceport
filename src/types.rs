use bitflags::bitflags;

pub type NodeId = u16;
pub type PacketId = u16;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Ping = 0x01,
    Pong = 0x02,
    UserData = 0x10,
    RoutingAnnounce = 0x20,
    RoutingQuery = 0x21,
    TimeSync = 0x30,
    DebugText = 0xF0,
}

impl MessageType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x01 => Some(Self::Ping),
            0x02 => Some(Self::Pong),
            0x10 => Some(Self::UserData),
            0x20 => Some(Self::RoutingAnnounce),
            0x21 => Some(Self::RoutingQuery),
            0x30 => Some(Self::TimeSync),
            0xF0 => Some(Self::DebugText),
            _ => None,
        }
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Flags: u8 {
        const ACK_REQUIRED = 1 << 0;
        const IS_ACK       = 1 << 1;
        const IS_ROUTED    = 1 << 2;
        const FRAGMENTED   = 1 << 3;
        const ERROR        = 1 << 4;
    }
}
