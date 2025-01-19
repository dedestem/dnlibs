use std::fs::File;
use std::io::{self, Read};

use crate::sha256::hash_sha256;

pub fn hash_sha256_file(path: &str) -> io::Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(hash_sha256(&buffer))
}
