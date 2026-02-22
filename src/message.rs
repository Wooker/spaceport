#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Message {
    Invoke = 1,
    Reply,
    Busy,
    Error,
    KernelVersion,
    Unknown,
}

impl From<u8> for Message {
    fn from(value: u8) -> Self {
        match value {
            1 => Message::Invoke,
            2 => Message::Reply,
            3 => Message::Busy,
            4 => Message::Error,
            5 => Message::KernelVersion,
            _ => Message::Unknown,
        }
    }
}

impl Into<u8> for Message {
    fn into(self) -> u8 {
        match self {
            Message::Invoke => 1,
            Message::Reply => 2,
            Message::Busy => 3,
            Message::Error => 4,
            Message::KernelVersion => 5,
            Message::Unknown => u8::MAX,
        }
    }
}
