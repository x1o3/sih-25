use crate::{
    error::Result,
    models::{ApiResponse, ExampleRequest, ExampleResponse},
};
use axum::{extract::Path, Json};

// get
pub async fn get_example(Path(id): Path<String>) -> Result<Json<ApiResponse<String>>> {
    Ok(Json(ApiResponse::new(format!("Retrieved item: {}", id))))
}

// post
pub async fn post_example(
    Json(payload): Json<ExampleRequest>,
) -> Result<Json<ApiResponse<ExampleResponse>>> {
    let response = ExampleResponse {
        success: true,
        message: format!("Received data: {}", payload.data),
    };

    Ok(Json(ApiResponse::new(response)))
}

// err
pub async fn error_example() -> Result<Json<ApiResponse<String>>> {
    Err(crate::error::AppError::BadRequest(
        "This is an example error".to_string(),
    ))
}
