use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: i32,
    pub task: String,
}
