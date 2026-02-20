use crate::message::Message;

use super::{
    constants::*,
    error::NodeError,
    packet::Packet,
    transport::Transport,
    types::{Flags, NodeId, PacketId},
};

pub struct Node<T: Transport> {
    pub node_id: NodeId,
    transport: T,
    next_packet_id: PacketId,
}

impl<T: Transport> Node<T> {
    pub fn new(node_id: NodeId, transport: T) -> Self {
        Self {
            node_id,
            transport,
            next_packet_id: 0,
        }
    }

    pub fn send(
        &mut self,
        dst: NodeId,
        msg_type: Message,
        payload: &[u8],
        flags: Flags,
        tx_buf: &mut [u8],
    ) -> Result<PacketId, NodeError> {
        let pid = self.next_packet_id;
        self.next_packet_id = self.next_packet_id.wrapping_add(1);

        let pkt = Packet {
            version: PROTOCOL_VERSION,
            flags,
            packet_id: pid,
            src: self.node_id,
            dst,
            ttl: MAX_TTL,
            msg_type,
            payload,
        };

        let len = pkt.encode(tx_buf).map_err(|_| NodeError::Encode)?;
        self.transport
            .write(&tx_buf[..len])
            .map_err(|_| NodeError::Transport)?;

        Ok(pid)
    }

    pub fn poll<'a>(
        &mut self,
        rx_buf: &mut [u8],
        payload_buf: &'a mut [u8],
    ) -> Result<Option<Packet<'a>>, NodeError> {
        // Read raw bytes from transport
        let n = self
            .transport
            .read(rx_buf)
            .map_err(|_| NodeError::Transport)?;

        // No data available
        if n == 0 {
            return Ok(None);
        }

        // Decode the packet into the provided payload buffer
        let pkt = Packet::decode(&rx_buf[..n], payload_buf).map_err(|_| NodeError::Decode)?;

        // Optional: drop packets with TTL = 0 (forwarding logic)
        if pkt.ttl == 0 {
            return Ok(None);
        }

        Ok(Some(pkt))
    }
}
