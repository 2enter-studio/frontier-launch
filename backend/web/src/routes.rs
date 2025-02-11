use crate::handlers::cargo::*;
use crate::handlers::news::get_news;
use crate::handlers::sys_info::get_temperature;
use crate::handlers::ws::ws_handler;
use crate::state::AppState;
use axum::routing::{any, get, post};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
// use tower_http::cors::CorsLayer;

pub fn get_routes(state: AppState) -> Router {
    // let cors = CorsLayer::permissive();
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/cargo",
                    Router::new()
                        .route("/metadata", post(send_cargo_metadata).get(get_cargo_ids))
                        .route("/image", post(send_cargo_image)),
                )
                .route("/news", get(get_news))
                .route("/sys-temp", get(get_temperature))
                .nest_service("/storage", ServeDir::new("../db/storage")),
        )
        .route("/ws", any(ws_handler))
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("../../frontend/build"))
        // .layer(cors)
        .with_state(state)
}
