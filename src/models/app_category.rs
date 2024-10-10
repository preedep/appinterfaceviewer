use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AppCategory {
    #[serde(rename = "category_id")]
    pub category_id : i32,
    #[serde(rename = "category_name")]
    pub category_name : String,
}