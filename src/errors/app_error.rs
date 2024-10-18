use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AppError {
    message : String
}
impl AppError {
    pub fn new(message : String) -> AppError {
        AppError {
            message
        }
    }
}