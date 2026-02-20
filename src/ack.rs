use super::types::PacketId;

#[derive(Copy, Clone)]
pub struct AckEntry {
    pub packet_id: PacketId,
    pub retries: u8,
}

pub struct AckTable<const N: usize> {
    entries: [Option<AckEntry>; N],
}

impl<const N: usize> AckTable<N> {
    pub const fn new() -> Self {
        Self { entries: [None; N] }
    }

    pub fn add(&mut self, entry: AckEntry) {
        self.entries[0] = Some(entry);
    }
}
