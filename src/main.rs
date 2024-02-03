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
use anyhow::anyhow;
use shuttle_secrets::SecretStore;

use tamagotch_frame::handlers;

const MAX_NFTS: u32 = 1000;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore,) -> shuttle_axum::ShuttleAxum {
    // Read the database URL from the environment variable
    let database_url = if let Some(secret) = secret_store.get("DATABASE_URL") {
        secret
    } else {
        return Err(anyhow!("DATABASE_URL was not found").into());
    };

    // Create a new database connection
    let conn = sea_orm::Database::connect(&database_url).await.expect("Unable to connect to database");

    // Run the migrations
    Migrator::up(&conn, None).await.expect("DB Migration failed");

    let neynar_api_key = if let Some(secret) = secret_store.get("NEYNAR_API_KEY") {
        secret
    } else {
        return Err(anyhow!("NEYNAR_API_KEY was not found").into());
    };

    std::env::set_var("NEYNAR_API_KEY", neynar_api_key);

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
