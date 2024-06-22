use std::sync::Arc;

use axum::{
    routing::{get, IntoMakeService},
    Router,
};
use utoipa::{openapi, OpenApi};
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub struct AppState {}

pub fn make_service(state: AppState) -> IntoMakeService<Router> {
    Router::new().merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(hello_world))
        .with_state(Arc::new(state))
        .into_make_service()
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
