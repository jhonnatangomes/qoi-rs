mod chunk;
mod decoder;
mod header;

pub use crate::decoder::decode;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
type Pixel = (u8, u8, u8, u8);
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub pixel: Pixel,
}

// pub struct Pixel {
//     color: Rgba,
// }

// impl Default for Pixel {
//     fn default() -> Self {
//         (0, 0, 0, 255)
//     }
// }

// impl From<(u32, u32, u8, u8, u8, u8)> for Point {
//     fn from(value: (u32, u32, u8, u8, u8, u8)) -> Self {
//         Self {
//             x: value.0,
//             y: value.1,
//             color: (value.2, value.3, value.4, value.5),
//         }
//     }
// }
// impl From<(u8, u8, u8, u8)> for Pixel {
//     fn from(color: (u8, u8, u8, u8)) -> Self {
//         Self { color }
//     }
// }
