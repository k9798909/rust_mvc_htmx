use axum::{
    routing::{delete, get, post},
    Router,
};
use db::create_connection_pool;
use dotenv::dotenv;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod common;
pub mod controllers;
pub mod db;
pub mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_mvc_web=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let pool = create_connection_pool().await;

    let app = Router::new()
        .route("/", get(controllers::movie_controller::list))
        .route("/list", get(controllers::movie_controller::list))
        .route("/list/all", get(controllers::movie_controller::all_list))
        .route("/list/search", post(controllers::movie_controller::search))
        .route("/add", get(controllers::movie_controller::add))
        .route("/add", post(controllers::movie_controller::insert))
        .route("/:id/edit", get(controllers::movie_controller::edit))
        .route("/:id/update", post(controllers::movie_controller::update))
        .route("/movie/:id", delete(controllers::movie_controller::delete))
        .with_state(pool)
        .route(
            "/assets/*path",
            get(controllers::static_resource::handle_assets),
        );

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(controllers::static_resource::handle_404);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
