use serde::{Deserialize, Serialize};
use crate::models::app_info::AppInfo;
use crate::models::com_method_info::CommunicationMethodInfo;
use crate::models::com_method_type::CommunicationMethodType;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AppConnectInfo {
    #[serde(rename = "app_start")]
    pub app_start : AppInfo,
    #[serde(rename = "app_end")]
    pub app_end : AppInfo,
    #[serde(rename = "communication_method_type")]
    pub communication_method_type: CommunicationMethodType,
    #[serde(rename = "communication_method_info")]
    pub communication_method_info: CommunicationMethodInfo,
}