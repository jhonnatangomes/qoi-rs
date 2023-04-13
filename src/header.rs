pub struct Header {
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
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
        let channels = &data[12..13];
        let colorspace = &data[13..14];
        Ok(Header {
            width: u32::from_be_bytes(width.try_into().unwrap()),
            height: u32::from_be_bytes(height.try_into().unwrap()),
            channels: channels[0],
            colorspace: colorspace[0],
        })
    }
}
