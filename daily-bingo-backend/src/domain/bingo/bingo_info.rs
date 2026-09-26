use super::BingoCellType;

pub struct BingoInfo {
    pub cols: usize,
    pub rows: usize,
    pub values: Vec<BingoCellType>,
}
