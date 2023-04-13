use crate::Point;

pub enum Chunk {
    QoiOpRgb(u8, u8, u8, u8),
    QoiOpRgba(u8, u8, u8, u8, u8),
    QoiOpIndex(u8),
    QoiOpDiff(u8),
    QoiOpLuma(u8, u8),
    QoiOpRun(u8),
}

impl Chunk {
    pub fn decode() -> Point {
        todo!()
    }
}

impl TryFrom<Vec<u8>> for Chunk {
    type Error = String;
    fn try_from(value: Vec<u8>) -> Result<Chunk, Self::Error> {
        let first = match value.first() {
            Some(x) => *x,
            None => return Err("Invalid Chunk".to_string()),
        };
    }
}
