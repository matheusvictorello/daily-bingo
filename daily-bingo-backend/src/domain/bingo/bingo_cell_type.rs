use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BingoCellType {
    Gap,
    Empty,
    Filled(String),
}
