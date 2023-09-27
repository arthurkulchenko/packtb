use serde::{Serialize, Deserialize};
use crate::error::BlobError;

pub fn read_u64<R: std::io::Read + std::fmt::Debug>(read_source: &mut R) -> Result<u64, BlobError> {
    let mut buffer = [0u8; 8];
    read_source.read_exact(&mut buffer)?;
    Ok(bincode::deserialize(&buffer)?)
}

pub fn write_u64<W: std::io::Write>(w: &mut W, data: u64) -> Result<(), BlobError> {
    let ec = bincode::serialize(&data)?;
    Ok(w.write_all(&ec)?)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Blob {
    k: Vec<u8>,
    v: Vec<u8>
}

impl Blob {
    pub fn serialize<K: Serialize, V: Serialize>(k: &K, v: &V) -> Result<Blob, bincode::Error> {
        // let k:i32 = 87;
        // let v = "hello W";
        Ok(Blob { k: bincode::serialize(k)?, v: bincode::serialize(v)? })
    }

    //             fin implementing Read trait
    pub fn read<R: std::io::Read + std::fmt::Debug>(read_source: &mut R) -> Result<Blob, BlobError> {
        let klen = read_u64(read_source)? as usize;
        let vlen = read_u64(read_source)? as usize;
        let mut k = vec![0u8; klen];
        let mut v = vec![0u8; vlen];
        read_source.read_exact(&mut k)?;
        read_source.read_exact(&mut v)?;
        Ok(Blob {k, v})
    }

    pub fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), BlobError> {
        // self.k:i32 = 87;
        // self.v = "hello W";
        let klen = bincode::serialize(&self.k.len())?;
        let vlen = bincode::serialize(&self.v.len())?;
        println!(" klen: {:?}\n vlen: {:?}\n k: {:?}\n v: {:?}", klen, vlen, self.k, self.v);
        // w.write_all(&klen)?;
        // w.write_all(&vlen)?;
        // w.write_all(&self.k)?;
        // w.write_all(&self.v)?;
        let data: Vec<_> = klen.iter()
                    .chain(vlen.iter())
                    .chain(self.k.iter())
                    .chain(self.v.iter())
                    .cloned()
                    .collect();
        println!("data: {:?}", data);
        w.write_all(&data)?;
        Ok(())
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

    pub fn get_v<'a, V: Deserialize<'a>>(&'a self) -> Result<V, BlobError> {
        Ok(bincode::deserialize(&self.v)?)
    }
}

pub fn get_all<R: std::io::Read + std::fmt::Debug>(read_source: &mut R) -> Result<String, failure::Error> {
    let mut buffer = vec![0u8; 35];
    read_source.read(&mut buffer)?;
    let res = bincode::deserialize(&buffer)?;
    println!("reees=========={:?}", res);
    Ok(res)
}

#[cfg(test)]
mod specs {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Point<T> {
        x: T,
        y: T
    }

    #[test]
    fn read_write_string() {
        let filename = "./test_data/read_write_string";
        if let Some(parent) = std::path::Path::new(filename).parent() {
           std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::remove_file(filename).ok();
        let k:i32 = 87;
        let v = "hello W";
        let serialized_blob = Blob::serialize(&k, &v).unwrap();
        {
            let mut file_out = std::fs::OpenOptions::new().write(true).create(true).open(filename).unwrap();
            let _result = serialized_blob.write(&mut file_out).unwrap();
        }
        let mut file = std::fs::File::open(filename).unwrap();

        let b2 = Blob::read(&mut file).unwrap();
        let v2: String = b2.get_v().unwrap();
        assert_eq!(&v2, v);
        let res = get_all(&mut file).unwrap();
        println!("res ============== {:?}", res);

        let p: Point<i32> = b2.get_v().unwrap();
        assert_eq!(p, Point {x: 7, y: 0});
    }

}
