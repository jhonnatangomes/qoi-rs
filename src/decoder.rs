use std::{fs, path::PathBuf};

use crate::header::Header;
use crate::Result;

pub fn decode(file: PathBuf) -> Result<()> {
    let file = fs::read_to_string(file).unwrap();
    let data = file.into_bytes();
    let header = match Header::new(&data) {
        Ok(header) => header,
        Err(e) => return Err(e),
    };
    Ok(())
}
