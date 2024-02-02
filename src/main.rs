use axum::{
    Json,
    response::Html,
    routing::{get, post},
    Router,
};

use tower_http::{
    services::{ServeDir},
};

use serde::Deserialize;

// todo: create different svg image states for the tamagotchi state

// todo: update struct to match payload structure

#[derive(Deserialize)]
struct CastId {
    fid: u32,
    hash: String,
}

#[derive(Deserialize)]
struct UntrustedData {
    fid: u32,
    url: String,
    messageHash: String,
    timestamp: u64,
    network: u8,
    buttonIndex: u8,
    castId: CastId,
}

#[derive(Deserialize)]
struct TrustedData {
    messageBytes: String,
}

#[derive(Deserialize)]
struct FrameData {
    untrustedData: UntrustedData,
    trustedData: TrustedData,
}

/// Generates HTML response for the tamagotchi frame with a dynamic image.
///
/// # Arguments
///
/// * `image_url` - A string slice that holds the URL of the image to be displayed.
async fn generate_html_response(image_url: &str) -> Html<String> {
    // Create the HTML content, interpolating the `image_url` where needed
    let html_content = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Tamagotchi Frame</title>
                <meta property="og:title" content="Tamagotchi Frame" />
                <meta property="og:image" content="{image_url}" />
                <meta property="fc:frame" content="vNext" />
                <meta property="fc:frame:image" content="{image_url}" />
                <meta property="fc:frame:button:1" content="Feed" />
                <meta property="fc:frame:button:2" content="Sleep" />
                <meta property="fc:frame:button:3" content="Clean" />
                <meta property="fc:frame:button:4" content="Play" />
                <meta property="fc:frame:post_url" content="https://tamagotch-frame.shuttleapp.rs/api/frame" />
            </head>
            <body>
                <h1>Tamagotchi Frame</h1>
                <img src="{image_url}" alt="Tamagotchi" />
            </body>
        </html>"#,
        image_url = image_url
    );

    Html(html_content)
}

async fn initial_frame() -> Html<String> {
    generate_html_response("https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg").await
}

async fn handle_button_click(Json(payload): Json<FrameData>) -> Html<String> {
    // todo: validate message

    // todo: get the button index
    let button_index = payload.untrustedData.buttonIndex;

    // todo: update the tamagotchi state in the db
    match button_index {
        1 => {
            // todo: feed the tamagotchi
        }
        2 => {
            // todo: put the tamagotchi to sleep
        }
        3 => {
            // todo: clean the tamagotchi
        }
        4 => {
            // todo: play with the tamagotchi
        }
        _ => {}
    }

    generate_html_response("https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg").await
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // todo: turn each route into a function
    let router = Router::new()
        .route("/", get( initial_frame))
        .route("/api/frame", post(handle_button_click))
        .nest_service("/public", ServeDir::new("public"));

    // todo: initialize the db

    Ok(router.into())
}
