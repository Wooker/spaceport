use crate::message::Message;

use super::{
    constants::*,
    crc::crc16_ccitt,
    error::{DecodeError, EncodeError},
    frame::{escape, unescape},
    types::{Flags, NodeId, PacketId},
};

pub const HEADER_LEN: usize = 10;

#[derive(Debug, Copy, Clone)]
pub struct Packet<'a> {
    pub version: u8,
    pub flags: Flags,
    pub packet_id: PacketId,
    pub src: NodeId,
    pub dst: NodeId,
    pub ttl: u8,
    pub msg_type: Message,
    pub payload: &'a [u8],
}

impl<'a> Packet<'a> {
    /// Encode packet into out buffer, returning number of bytes written
    pub fn encode(&self, out: &mut [u8]) -> Result<usize, EncodeError> {
        // Temporary buffer for raw header + payload + CRC before escaping
        let mut raw_buf = [0u8; 512]; // adjust size as needed
        let mut idx = 0;

        // Header
        raw_buf[idx] = self.version;
        raw_buf[idx + 1] = self.flags.bits();
        raw_buf[idx + 2..idx + 4].copy_from_slice(&self.packet_id.to_be_bytes());
        raw_buf[idx + 4..idx + 6].copy_from_slice(&self.src.to_be_bytes());
        raw_buf[idx + 6..idx + 8].copy_from_slice(&self.dst.to_be_bytes());
        raw_buf[idx + 8] = self.ttl;
        raw_buf[idx + 9] = self.msg_type as u8;
        idx += HEADER_LEN;

        // Payload
        let payload_len = self.payload.len();
        if idx + payload_len + 2 > raw_buf.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        raw_buf[idx..idx + payload_len].copy_from_slice(self.payload);
        idx += payload_len;

        // CRC16
        let crc = crc16_ccitt(&raw_buf[..idx]);
        raw_buf[idx..idx + 2].copy_from_slice(&crc.to_be_bytes());
        idx += 2;

        // Escape everything
        if out.len() < idx * 2 + 2 {
            // worst case: every byte is escaped + SOF/EOF
            return Err(EncodeError::BufferTooSmall);
        }

        let mut out_idx = 0;
        out[out_idx] = SOF;
        out_idx += 1;

        let escaped_len = escape(&raw_buf[..idx], &mut out[out_idx..])
            .map_err(|_| EncodeError::BufferTooSmall)?;
        out_idx += escaped_len;

        out[out_idx] = EOF;
        out_idx += 1;

        Ok(out_idx)
    }

    pub fn decode(buf: &[u8], payload_buf: &'a mut [u8]) -> Result<Packet<'a>, DecodeError> {
        // check SOF/EOF
        if buf.len() < 3 || buf[0] != SOF || *buf.last().unwrap() != EOF {
            return Err(DecodeError::InvalidFrame);
        }

        let mut raw_buf = [0u8; 512];
        let unescaped_len = unescape(&buf[1..buf.len() - 1], &mut raw_buf)
            .map_err(|_| DecodeError::InvalidFrame)?;

        if unescaped_len < HEADER_LEN + 2 {
            return Err(DecodeError::InvalidFrame);
        }

        let data_len = unescaped_len - 2; // exclude CRC
        let recv_crc = u16::from_be_bytes([raw_buf[data_len], raw_buf[data_len + 1]]);
        let calc_crc = super::crc::crc16_ccitt(&raw_buf[..data_len]);
        if recv_crc != calc_crc {
            return Err(DecodeError::BadCrc);
        }

        let payload_len = data_len - HEADER_LEN;
        if payload_len > payload_buf.len() {
            return Err(DecodeError::InvalidFrame);
        }
        payload_buf[..payload_len].copy_from_slice(&raw_buf[HEADER_LEN..data_len]);

        let packet = Packet {
            version: raw_buf[0],
            flags: super::types::Flags::from_bits_truncate(raw_buf[1]),
            packet_id: PacketId::from_be_bytes([raw_buf[2], raw_buf[3]]),
            src: NodeId::from_be_bytes([raw_buf[4], raw_buf[5]]),
            dst: NodeId::from_be_bytes([raw_buf[6], raw_buf[7]]),
            ttl: raw_buf[8],
            msg_type: Message::from(raw_buf[9]),
            payload: &payload_buf[..payload_len],
        };

        Ok(packet)
    }

    pub fn reply(&self, payload: &'a [u8]) -> Packet<'a> {
        Packet {
            version: PROTOCOL_VERSION,
            flags: Flags::empty(),
            packet_id: self.packet_id + 1,
            src: self.dst,
            dst: self.src,
            ttl: self.ttl,
            msg_type: Message::Reply,
            payload,
        }
    }
}
