use axum::extract::State;
use axum::Json;

use crate::domain::models::customer::PostError;
use crate::handlers::customers::{CreatePostRequest, PostResponse};
use crate::infra::repositories::customer_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

pub async fn create_customer(
    State(state): State<AppState>,
    JsonExtractor(new_customer): JsonExtractor<CreatePostRequest>,
) -> Result<Json<customerResponse>, PostError> {
    let new_customer_db = post_repository::NewPostDb {
        first_name: new_customer.first_name,
        last_name: new_customer.last_name,
        organization_name: new_customer.organization_name,
    };

    let created_customer = post_repository::insert(&state.pool, new_post_db)
        .await
        .map_err(customerError::InfraError)?;

    let customer_response = PostResponse {
        id: created_customer.id,
        first_name: new_customer.first_name,
        last_name: new_customer.last_name,
        organization_name: new_customer.organization_name,
    };

    Ok(Json(customer_response))
}
