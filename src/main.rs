use axum::{
    routing::{get, post},
    Router,
};

use tower_http::{
    services::{ServeDir},
};

use migration::{Migrator, MigratorTrait};

use dotenv::dotenv;
use std::env;

use tamagotch_frame::handlers;

const MAX_NFTS: u32 = 1000;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // Load environment variables from .env file if present
    dotenv().ok();

    // Read the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Create a new database connection
    let conn = sea_orm::Database::connect(&database_url).await.expect("Unable to connect to database");

    // Run the migrations
    Migrator::up(&conn, None).await.expect("DB Migration failed");

    // create a new router and register the routes
    let router = Router::new()
        .route("/", get(handlers::initial_frame))
        .route("/api/connect", post(handlers::connect_tamagotchi))
        .route("/api/actions", post(handlers::handle_action_click))
        .route("/api/tamagotchi/:fid", get(handlers::get_tamagotchi))
        .route("/api/guess/:count/:result", post(handlers::guessing_game))
        .route("/api/guessing_game_result/:result", post(handlers::guessing_game_result))
        .with_state(conn)
        .nest_service("/public", ServeDir::new("public"));

    Ok(router.into())
}
