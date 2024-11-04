use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageView {
    pub path: String,
    pub views: u64,
}
