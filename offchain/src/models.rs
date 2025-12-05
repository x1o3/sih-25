use serde::{Deserialize, Serialize};

/// Example request model
#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleRequest {
    pub data: String,
}

/// Example response model
#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleResponse {
    pub success: bool,
    pub message: String,
}

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
