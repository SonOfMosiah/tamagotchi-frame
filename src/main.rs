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

use tamagotch_frame::generate_svg_with_color;

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

const MAX_NFTS: u32 = 1000;

/// Generates HTML response for the tamagotchi frame with a dynamic image, dynamic buttons, and dynamic post URL.
///
/// # Arguments
///
/// * `image_url` - A string slice that holds the URL of the image to be displayed.
/// * `button_names` - A slice of string slices containing the names of the buttons to be displayed.
/// * `post_url` - A string slice that holds the URL where the form data should be posted.
async fn generate_html_response(image_url: &str, button_names: &[&str], post_url: &str) -> Html<String> {
    // Ensure that there are no more than four buttons
    let buttons = button_names.iter().take(4).enumerate().map(|(index, name)| {
        format!(r#"<meta property="fc:frame:button:{}" content="{}" />"#, index + 1, name)
    }).collect::<Vec<String>>().join("\n                ");

    // Create the HTML content, interpolating the `image_url`, `buttons`, and `post_url` where needed
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
                {buttons}
                <meta property="fc:frame:post_url" content="{post_url}" />
            </head>
            <body>
                <h1>Tamagotchi Frame</h1>
                <img src="{image_url}" alt="Tamagotchi" />
            </body>
        </html>"#,
        image_url = image_url,
        buttons = buttons,
        post_url = post_url
    );

    Html(html_content)
}

async fn initial_frame() -> Html<String> {
    let image_url = "https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg";
    // todo: svg should be dynamically generated with color and pet based on the fid
    let image_url = generate_svg_with_color("cccccc", 1);

    let button_names = ["Feed", "Sleep", "Clean", "Play"];
    let post_url = "https://tamagotch-frame.shuttleapp.rs/api/frame";
    generate_html_response(&image_url, &button_names, post_url).await
}

async fn create_tamagotchi(Json(payload): Json<FrameData>) -> Html<String> {
    // todo: create a new tamagotchi in the db
    // todo: mint a new tamagotchi NFT if one of first MAX_NFTS is created

    let fid = payload.untrustedData.fid;

    // todo: check if fid already has a tamagotchi


    let image_url = "https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg";

    // todo: svg should be dynamically generated with color and pet based on the fid
    let image_url = generate_svg_with_color("cccccc", 1);

    let button_names = ["Feed", "Sleep", "Clean", "Play"];
    let post_url = format!("https://tamagotch-frame.shuttleapp.rs/api/frame/{fid}");
    generate_html_response(&image_url, &button_names, &post_url).await
}

async fn handle_action_click(Json(payload): Json<FrameData>) -> Html<String> {
    // todo: validate message

    let button_index = payload.untrustedData.buttonIndex;
    let fid = payload.untrustedData.fid;

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

    let image_url = "https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg";
    let button_names = ["Feed", "Sleep", "Clean", "Play"];
    let post_url = "https://tamagotch-frame.shuttleapp.rs/api/frame";
    generate_html_response(image_url, &button_names, post_url).await
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // todo: turn each route into a function
    let router = Router::new()
        .route("/", get( initial_frame))
        .route("/api/create", post(create_tamagotchi))
        .route("/api/actions", post(handle_action_click))
        .nest_service("/public", ServeDir::new("public"));

    // todo: initialize the db

    Ok(router.into())
}
