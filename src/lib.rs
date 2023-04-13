mod chunk;
mod decoder;
mod header;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
type Rgba = (u8, u8, u8, u8);
pub struct Point {
    x: u32,
    y: u32,
    color: Rgba,
}
