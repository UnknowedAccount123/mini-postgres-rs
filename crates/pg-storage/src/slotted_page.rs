pub const PAGE_SIZE: usize = 8192;

#[derive(Debug, Clone)]
pub struct Slot {
    pub offset: u16,
    pub len: u16,
}

pub struct SlottedPage {
    pub data: Vec<u8>,
    pub slots: Vec<Slot>,
}

impl SlottedPage {
    pub fn new() -> Self {
        Self { data: vec![0; PAGE_SIZE], slots: vec![] }
    }

    pub fn insert(&mut self, tuple: &[u8]) -> Option<usize> {
        let offset = self.slots.len() as u16;
        self.slots.push(Slot { offset, len: tuple.len() as u16 });
        Some(self.slots.len() - 1)
    }
}
