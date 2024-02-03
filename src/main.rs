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

use tamagotch_frame::handlers::{initial_frame, connect_tamagotchi, handle_action_click, get_tamagotchi};

const MAX_NFTS: u32 = 1000;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // Load environment variables from .env file if present
    dotenv().ok();

    // Read the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // todo: better error handling
    let conn = sea_orm::Database::connect(&database_url).await.unwrap();
    // todo: better error handling
    Migrator::up(&conn, None).await.unwrap();

    // create a new router and register the routes
    let router = Router::new()
        .route("/", get( initial_frame))
        .route("/api/connect", post(connect_tamagotchi))
        .route("/api/actions", post(handle_action_click))
        .route("/api/tamagotchi/:fid", get(get_tamagotchi))
        .with_state(conn)
        .nest_service("/public", ServeDir::new("public"));

    Ok(router.into())
}
