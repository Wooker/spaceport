#![no_std]

pub mod ack;
pub mod constants;
pub mod crc;
pub mod error;
pub mod frame;
pub mod message;
pub mod node;
pub mod packet;
pub mod router;
pub mod transport;
pub mod types;

#[allow(unused_imports)]
mod tests {
    use crate::{
        constants::PROTOCOL_VERSION,
        error::EncodeError,
        message::Message::Invoke,
        packet::{HEADER_LEN, MAX_BUFFER_LENGTH, MAX_PACKET_LENGTH, MAX_PAYLOAD_LENGTH, Packet},
        types::Flags,
    };

    #[test]
    fn encode_empty() {
        let payload = &[];
        let packet = Packet {
            version: PROTOCOL_VERSION,
            flags: Flags::empty(),
            packet_id: 0,
            src: 0,
            dst: 0,
            ttl: 0,
            msg_type: Invoke,
            payload,
        };
        let mut out = [0; MAX_BUFFER_LENGTH];
        let encoded = packet.encode(&mut out);
        assert_eq!(encoded.is_ok(), true);
        let size = encoded.unwrap();
        let mut in_buf = [0; MAX_BUFFER_LENGTH];
        let decoded = Packet::decode(&out[..size], &mut in_buf);
        assert_eq!(decoded.is_ok(), true);
        let decoded = decoded.unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn encode_decode_full_unescaped() {
        let payload = &[0; MAX_PAYLOAD_LENGTH];
        let packet = Packet {
            version: PROTOCOL_VERSION,
            flags: Flags::empty(),
            packet_id: 0,
            src: 0,
            dst: 0,
            ttl: 0,
            msg_type: Invoke,
            payload,
        };
        let mut out = [0; MAX_BUFFER_LENGTH];
        let encoded = packet.encode(&mut out);
        assert_eq!(encoded.is_ok(), true);
        let size = encoded.unwrap();
        let mut in_buf = [0; MAX_BUFFER_LENGTH];
        let decoded = Packet::decode(&out[..size], &mut in_buf);
        assert_eq!(decoded.is_ok(), true);
        let decoded = decoded.unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn encode_decode_full_escaped() {
        let payload = &[0x7f; MAX_PAYLOAD_LENGTH];
        let packet = Packet {
            version: PROTOCOL_VERSION,
            flags: Flags::empty(),
            packet_id: 0,
            src: 0,
            dst: 0,
            ttl: 0,
            msg_type: Invoke,
            payload,
        };
        let mut out = [0; MAX_BUFFER_LENGTH];
        let encoded = packet.encode(&mut out);
        assert_eq!(encoded.is_ok(), true);
        let size = encoded.unwrap();
        let mut in_buf = [0; MAX_BUFFER_LENGTH];
        let decoded = Packet::decode(&out[..size], &mut in_buf);
        assert_eq!(decoded.is_ok(), true);
        let decoded = decoded.unwrap();
        assert_eq!(packet, decoded);
    }
}
