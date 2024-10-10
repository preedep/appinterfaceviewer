use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunicationMethodType {
    #[serde(rename = "com_method_type_id")]
    pub com_method_type_id: i32,
    #[serde(rename = "com_method_name")]
    pub com_method_type_name: String,
}