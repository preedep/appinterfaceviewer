use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize,Clone)]
pub struct CommunicationMethodInfo {
    #[serde(rename = "com_method_id")]
    pub com_method_id: i32,
    #[serde(rename = "kafka_topic")]
    pub kafka_topic: Option<String>,
    #[serde(rename = "rest_api_http_method")]
    pub rest_api_method: Option<String>,
    #[serde(rename = "rest_api_http_uri")]
    pub rest_api_endpoint: Option<String>,
}