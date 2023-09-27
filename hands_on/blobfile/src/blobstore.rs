use serde::{Serialize, Deserialize};
use std::io::{Seek, SeekFrom};
use std::fs::{File,OpenOptions};
use crate::error::BlobError;
use crate::blob::{read_u64, write_u64, Blob};

pub const CONT_SIZE: u64 = 32;

pub struct BlobStore {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
    elements: u64
}

impl BlobStore {
    pub fn new(fname: &str, block_size: u64, nblocks: u64) -> Result<Self, BlobError> {
        let hseed = rand::random::<u64>();
        let mut ff = OpenOptions::new().create_new(true).write(true).read(true).open(fname)?;
        let f = &mut ff;
        f.set_len(CONT_SIZE + block_size * nblocks)?;
        f.seek(SeekFrom::Start(0))?;
        write_u64(f, hseed)?;
        write_u64(f, block_size)?;
        write_u64(f, nblocks)?;
        write_u64(f, 0)?;
        for x in 0..nblocks {
            f.seek(SeekFrom::Start(CONT_SIZE + x * block_size))?;
            write_u64(f, 0)?;
            write_u64(f, block_size - 16)?;
        }
        Ok(Self { hseed, file: ff, block_size, nblocks, elements: 0 })
    }

    pub fn open(fname: &str) -> Result<Self, BlobError> {
        let mut ff = OpenOptions::new().write(true).read(true).open(fname)?;
        let f = &mut ff;
        f.seek(SeekFrom::Start(0))?;
        let hseed = read_u64(f)?;
        let block_size = read_u64(f)?;
        let nblocks = read_u64(f)?;
        let elements = read_u64(f)?;
        Ok(Self { hseed, file: ff, block_size, nblocks, elements })
    }

    pub fn new_or_open(fname: &str, bsize: u64, nblocks: u64) -> Result<Self, BlobError> {
        Self::new(fname, bsize, nblocks).or_else(|_| Self::open(fname))
    }

    pub fn increment_elements(&mut self, n: i32) -> Result<(), BlobError> {
        if n > 0 {
            self.elements += n as u64;
        } else {
            let n2 = (-n) as u64;
            if self.elements > n2 { self.elements -= n2; }
        }
        self.file.seek(SeekFrom::Start(24))?;
        write_u64(&mut self.file, self.elements)?;
        Ok(())
    }

    fn insert_only<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<(), BlobError> {
        let blob = Blob::serialize(&k, &v)?;
        if blob.length() > self.block_size {
            return Err(BlobError::TooBig(blob.length()));
        }
        let bucket = blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;
        let mut position = f.seek(SeekFrom::Start(CONT_SIZE + self.block_size * bucket))?;
        loop {
            if position > CONT_SIZE + self.block_size * (bucket + 1) {
                return Err(BlobError::NoRoom);
            }
            let klen = read_u64(f)?;
            let vlen = read_u64(f)?;
            if klen == 0 && blob.length() < vlen {
                f.seek(SeekFrom::Start(position))?;
                blob.write(f)?;
                write_u64(f, 0)?;
                write_u64(f, (vlen - blob.length()) - 16)?;
                return Ok(());
            }
            position = f.seek(SeekFrom::Start(position + 16 + klen + vlen))?;
        }
    }

    pub fn b_start(&self, b: u64) -> u64 {
        CONT_SIZE + self.block_size * b
    }

    pub fn get<K: Serialize>(&mut self, k: &K) -> Result<Blob, BlobError> {
        let s_blob = Blob::serialize(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let b_start = self.b_start(bucket);
        let b_end = self.b_start(bucket + 1);
        let f = &mut self.file;
        let mut position = f.seek(SeekFrom::Start(b_start))?;
        loop {
            if position >= b_end {
                return Err(BlobError::NotFound);
            }

            let b = Blob::read(f)?;
            if b.key_match(&s_blob) { return Ok(b) }

            position += b.length();
        }
    }
}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    pub fn craete_file() {
        let fs = "test_data/craete_file";
        std::fs::remove_file(fs).ok();
        let bs = BlobStore::new(fs, 1000, 10).unwrap();
        let block_size = bs.block_size;
        let mut bs2 = BlobStore::open(fs).unwrap();
        assert_eq!(bs2.block_size, block_size);
        bs2.insert_only("fish", "so long and thanks for all teh fish").unwrap();
        bs2.insert_only(34, "rrrrrrrrrrr thanks for all teh fish").unwrap();
        bs2.insert_only("hello", 434).unwrap();
        drop(bs2);
        let mut b3 = BlobStore::open(fs).unwrap();
        assert_eq!(b3.get(&"hello").unwrap().get_v::<u16>().unwrap(), 434);
    }
}
