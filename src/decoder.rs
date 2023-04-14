use std::{fs, path::PathBuf};

use crate::chunk::Chunks;
use crate::header::Header;
use crate::{Point, Result};

pub fn decode(file: PathBuf) -> Result<Vec<Point>> {
    let file = fs::read(file)?;
    let header = match Header::new(&file) {
        Ok(header) => header,
        Err(e) => return Err(e),
    };
    let chunks = Chunks::new(file[14..].into(), header.width, header.height);
    chunks.decode()
}
