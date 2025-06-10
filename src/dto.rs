use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResultId {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateResult {
    pub result: bool,
}
