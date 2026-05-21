use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub const PAGE_SIZE: usize = 8192;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
    pub len: usize,
}

pub struct HeapFile {
    pub path: String,
}

impl HeapFile {
    pub fn insert(&self, bytes: &[u8]) {
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path).unwrap();
        f.write_all(bytes).unwrap();
    }

    pub fn read_all(&self) -> Vec<u8> {
        let mut f = File::open(&self.path).unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        buf
    }
}

pub struct WAL {
    pub path: String,
}

impl WAL {
    pub fn log(&self, entry: &str) {
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path).unwrap();
        writeln!(f, "{}", entry).unwrap();
    }
}
