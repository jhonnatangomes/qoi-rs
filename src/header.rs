#[derive(Debug)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}
use crate::Result;

impl Header {
    pub fn new(data: &Vec<u8>) -> Result<Header> {
        let magic_chars = &data[0..4];
        if magic_chars != b"qoif" {
            return Err("Invalid magic chars".into());
        }
        let width = &data[4..8];
        let height = &data[8..12];
        let channels = data[12..13][0];
        let colorspace = data[13..14][0];
        if channels != 3 && channels != 4 {
            return Err(format!("Invalid number of channels {channels}").into());
        }
        if colorspace != 0 && colorspace != 1 {
            return Err("Invalid colorspace value {colorspace}".into());
        }
        Ok(Header {
            width: u32::from_be_bytes(width.try_into().unwrap()),
            height: u32::from_be_bytes(height.try_into().unwrap()),
            channels,
            colorspace,
        })
    }
}
