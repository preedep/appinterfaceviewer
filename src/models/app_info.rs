use serde::{Deserialize, Serialize};
use crate::models::app_category::AppCategory;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AppInfo {
    #[serde(rename = "app_id")]
    pub app_id: String,
    #[serde(rename = "app_category")]
    pub app_category : AppCategory,
    #[serde(rename = "app_name")]
    pub app_name: String,
    #[serde(rename = "app_level")]
    pub app_level : u8
}