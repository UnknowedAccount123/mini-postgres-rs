use std::collections::HashMap;

pub type PageId = u64;

pub struct BufferFrame {
    pub data: Vec<u8>,
    pub dirty: bool,
    pub pin_count: u32,
}

pub struct BufferPool {
    pub frames: HashMap<PageId, BufferFrame>,
    pub capacity: usize,
}

impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        Self { frames: HashMap::new(), capacity }
    }

    pub fn fetch_page(&mut self, page_id: PageId) -> Option<&mut BufferFrame> {
        self.frames.get_mut(&page_id)
    }

    pub fn put_page(&mut self, page_id: PageId, frame: BufferFrame) {
        if self.frames.len() >= self.capacity {
            // naive eviction
            if let Some((&k, _)) = self.frames.iter().next() {
                self.frames.remove(&k);
            }
        }
        self.frames.insert(page_id, frame);
    }
}
