use std::fs::File;
use std::io::{self, Read};

use crate::sha256::sha256;

pub fn sha256file(path: &str) -> io::Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(sha256(&buffer))
}