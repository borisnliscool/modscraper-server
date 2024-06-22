use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::http::{HeaderValue, StatusCode};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::CLIENT;
use crate::models::GameMod;

#[derive(Deserialize)]
struct QueryParameters {
    query: String,
    page: Option<usize>,
}

#[derive(Serialize)]
struct SearchResponse {
    hits: Vec<GameMod>,
    total_hits: Option<usize>,
    total_pages: Option<usize>,
    current_page: Option<usize>,
    search_time: usize,
}

async fn search(query: Query<QueryParameters>) -> Result<Json<SearchResponse>, StatusCode> {
    let query: QueryParameters = query.0;
    let mods_index = CLIENT.index("mods");

    let result = match mods_index
        .search()
        .with_query(&query.query)
        .with_hits_per_page(24)
        .with_page(query.page.unwrap_or(1))
        .execute::<GameMod>()
        .await {
        Ok(result) => result,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(
        Json(SearchResponse {
            hits: result.hits.iter().map(|r| r.result.clone()).collect(),
            total_hits: result.total_hits,
            total_pages: result.total_pages,
            current_page: result.page,
            search_time: result.processing_time_ms,
        })
    )
}

async fn get_mod(Path(mod_id): Path<String>) -> Result<Json<GameMod>, StatusCode> {
    let mods_index = CLIENT.index("mods");

    match mods_index.get_document::<GameMod>(&mod_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

pub fn create() -> Router {
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/search", get(search))
        .route("/mod/:mod_id", get(get_mod))
        .layer(ServiceBuilder::new().layer(cors))
}