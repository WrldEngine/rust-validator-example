use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatusData {
    pub status: String,
    pub valid: bool,
}