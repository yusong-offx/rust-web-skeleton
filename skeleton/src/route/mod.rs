use std::sync::Arc;

use axum::{
    routing::{get, IntoMakeService},
    Router,
};

pub struct AppState {}

pub fn make_service(state: AppState) -> IntoMakeService<Router> {
    Router::new()
        .route("/", get(hello_world))
        .with_state(Arc::new(state))
        .into_make_service()
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
