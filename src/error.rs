#[derive(Debug)]
pub enum FrameError {
    BufferTooSmall,
    InvalidEscape,
}

#[derive(Debug, PartialEq)]
pub enum EncodeError {
    BufferTooSmall,
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidFrame,
    InvalidFrameUnescape,
    InvalidSOFEOF,
    BadCrc,
    UnknownMessageType,
}

#[derive(Debug)]
pub enum NodeError {
    Transport,
    Encode,
    Decode,
}
