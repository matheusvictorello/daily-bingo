use uuid::Uuid;
use super::BingoCellType;

pub struct BingoInfo {
    cols: usize,
    rows: usize,
    values: Vec<BingoCellType>,
}
