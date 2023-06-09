use crate::{Pixel, Point, Result};

pub struct Chunks {
    data: Vec<u8>,
    index: usize,
    width: u32,
    height: u32,
    previous_pixel: Pixel,
    previously_seen_pixels: [Pixel; 64],
    points: Vec<Point>,
}

impl Chunks {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            index: 0,
            width,
            height,
            previous_pixel: (0, 0, 0, 255),
            previously_seen_pixels: [(0, 0, 0, 255); 64],
            points: vec![],
        }
    }
    pub fn decode(mut self) -> Result<Vec<Point>> {
        while let Some(byte) = self.next() {
            match byte {
                0b11111110 => self.qoi_op_rgb()?,
                0b11111111 => self.qoi_op_rgba()?,
                byte => match two_bit_tag(byte) {
                    0b0 => self.qoi_op_index(byte)?,
                    0b1 => self.qoi_op_diff(byte)?,
                    0b10 => self.qoi_op_luma(byte)?,
                    0b11 => self.qoi_op_run(byte)?,
                    _ => return Err(format!("Invalid Chunk Tag {byte}").into()),
                },
            };
        }
        let end_byte_stream = &self.data[self.index..];
        if end_byte_stream != &[0, 0, 0, 0, 0, 0, 0, 1] {
            return Err(format!("Invalid end byte stream {:?}", end_byte_stream).into());
        }
        Ok(self.points)
    }
    fn index_to_position(&self) -> (u32, u32) {
        (
            self.points.len() as u32 % self.width,
            self.points.len() as u32 / self.width + 1,
        )
    }
    fn next(&mut self) -> Option<u8> {
        if self.points.len() == (self.width * self.height) as usize {
            return None;
        }
        let byte = self.data[self.index];
        self.index += 1;
        Some(byte)
    }
    fn next_n_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![];
        for _ in 0..n {
            if let Some(byte) = self.next() {
                bytes.push(byte);
            } else {
                return Err(format!(
                    "Unexpected EOF at byte {}, position {}",
                    self.data[self.index], self.index
                )
                .into());
            }
        }
        Ok(bytes)
    }
    fn qoi_op_rgb(&mut self) -> Result<()> {
        let bytes = self.next_n_bytes(3)?;
        let pixel = (bytes[0], bytes[1], bytes[2], self.previous_pixel.3);
        self.add_point(pixel);
        Ok(())
    }
    fn qoi_op_rgba(&mut self) -> Result<()> {
        let bytes = self.next_n_bytes(4)?;
        let pixel = (bytes[0], bytes[1], bytes[2], bytes[3]);
        self.add_point(pixel);
        Ok(())
    }
    fn qoi_op_index(&mut self, byte: u8) -> Result<()> {
        let index = byte & 0b00111111;
        let pixel = self.previously_seen_pixels[index as usize];
        self.add_point(pixel);
        Ok(())
    }
    fn qoi_op_diff(&mut self, byte: u8) -> Result<()> {
        let dr = (byte >> 4 & 0b00000011).wrapping_sub(2);
        let dg = (byte >> 2 & 0b00000011).wrapping_sub(2);
        let db = (byte & 0b00000011).wrapping_sub(2);
        let pixel = (
            self.previous_pixel.0.wrapping_add(dr),
            self.previous_pixel.1.wrapping_add(dg),
            self.previous_pixel.2.wrapping_add(db),
            self.previous_pixel.3,
        );
        self.add_point(pixel);
        Ok(())
    }
    fn qoi_op_luma(&mut self, byte: u8) -> Result<()> {
        let dg = (byte & 0b00111111).wrapping_sub(32);
        let byte = self.next().ok_or(format!(
            "Unexpected EOF at byte {}, position {}",
            self.data[self.index], self.index
        ))?;
        let dr_dg = (byte >> 4 & 0b00001111).wrapping_sub(8);
        let db_dg = (byte & 0b00001111).wrapping_sub(8);
        let dr = dr_dg.wrapping_add(dg);
        let db = db_dg.wrapping_add(dg);
        let pixel = (
            self.previous_pixel.0.wrapping_add(dr),
            self.previous_pixel.1.wrapping_add(dg),
            self.previous_pixel.2.wrapping_add(db),
            self.previous_pixel.3,
        );
        self.add_point(pixel);
        Ok(())
    }
    fn qoi_op_run(&mut self, byte: u8) -> Result<()> {
        let repeat = (byte & 0b00111111) + 1;
        for _ in 0..repeat {
            self.add_point(self.previous_pixel);
        }
        Ok(())
    }
    fn add_point(&mut self, pixel: Pixel) {
        self.previous_pixel = pixel;
        self.previously_seen_pixels[index_hash(pixel)] = pixel;
        let position = self.index_to_position();
        self.points.push(Point {
            x: position.0,
            y: position.1,
            pixel,
        });
    }
}

fn two_bit_tag(byte: u8) -> u8 {
    byte >> 6
}

fn index_hash(pixel: Pixel) -> usize {
    let pixel = (
        pixel.0 as usize,
        pixel.1 as usize,
        pixel.2 as usize,
        pixel.3 as usize,
    );
    (pixel.0 * 3 + pixel.1 * 5 + pixel.2 * 7 + pixel.3 * 11) % 64
}
