use crate::{
    error::Result,
    models::{ApiResponse, ExampleRequest, ExampleResponse},
};
use axum::{extract::Path, Json};

/// Example GET handler with path parameter
pub async fn get_example(Path(id): Path<String>) -> Result<Json<ApiResponse<String>>> {
    Ok(Json(ApiResponse::new(format!("Retrieved item: {}", id))))
}

/// Example POST handler with JSON body
pub async fn post_example(
    Json(payload): Json<ExampleRequest>,
) -> Result<Json<ApiResponse<ExampleResponse>>> {
    let response = ExampleResponse {
        success: true,
        message: format!("Received data: {}", payload.data),
    };

    Ok(Json(ApiResponse::new(response)))
}

/// Example handler that returns an error
pub async fn error_example() -> Result<Json<ApiResponse<String>>> {
    Err(crate::error::AppError::BadRequest(
        "This is an example error".to_string(),
    ))
}
