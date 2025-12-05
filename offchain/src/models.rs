use serde::{Deserialize, Serialize};

// request
#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleRequest {
    pub data: String,
}

// response
#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleResponse {
    pub success: bool,
    pub message: String,
}

// API response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
