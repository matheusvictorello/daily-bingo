use serde::{Deserialize, Serialize};

use super::BingoCellType;

#[derive(Serialize, Deserialize)]
pub struct BingoInfo {
    pub cols: usize,
    pub rows: usize,
    pub values: Vec<BingoCellType>,
}

impl BingoInfo {
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(self.cols > 0 && self.rows > 0, "cols and rows must be positive");
        anyhow::ensure!(
            self.values.len() == self.cols * self.rows,
            "bingo has {} values, expected {}x{}",
            self.values.len(),
            self.cols,
            self.rows,
        );
        Ok(())
    }
}
