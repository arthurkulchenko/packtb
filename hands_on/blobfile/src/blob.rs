use serde::{Serialize, Deserialize};
use crate::error::BlobError;

pub fn read_u64<R: std::io::Read>(reference: &mut R) -> Result<u64, BlobError> {
    let mut buffer = [0u8, 8];
    reference.read_exact(&mut buffer)?;
    Ok(bincode::deserialize(&buffer)?)
}

pub fn write_u64<W: std::io::Write>(w: &mut W, data: u64) -> Result<(), BlobError> {
    let ec = bincode::serialize(&data)?;
    Ok(w.write_all(&ec)?)
}

pub struct Blob {
    k: Vec<u8>,
    v: Vec<u8>
}

impl Blob {
    pub fn from<K: Serialize, V: Serialize>(k: &K, v: &V) -> Result<Blob, bincode::Error> {
        Ok(Blob { k: bincode::serialize(k)?, v: bincode::serialize(v)? })
    }

    pub fn out<W: std::io::Write>(&self, w: &mut W) -> Result<(), BlobError> {
        let klen = bincode::serialize(&self.k.len())?;
        let vlen = bincode::serialize(&self.v.len())?;

        w.write_all(&klen)?;
        w.write_all(&vlen)?;
        w.write_all(&self.k)?;
        w.write_all(&self.v)?;
        Ok(())
    }

    pub fn read<R: std::io::Read>(r: &mut R) -> Result<Blob, failure::Error> {
        let klen = read_u64(r)? as usize;
        let vlen = read_u64(r)? as usize;
        let mut k = vec![0u8; klen];
        let mut v = vec![0u8; vlen];

        r.read_exact(&mut k)?;
        r.read_exact(&mut v)?;
        Ok(Blob {k, v})
    }

    pub fn get_v<'a, V: Deserialize<'a>>(&'a self) -> Result<V, BlobError> {
        Ok(bincode::deserialize(&self.v)?)
    }

    pub fn length(&self) -> u64 {
        (16 + self.k.len() + self.v.len()) as u64
    }

    pub fn k_hash(&self, seed: u64) -> u64 {
        hash::hash(seed, &self.k)
    }

    pub fn key_match(&self, rhs: &Self) -> bool {
        self.k == rhs.k
    }
}
