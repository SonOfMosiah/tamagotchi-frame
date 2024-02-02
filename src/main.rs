use axum::{
    Json,
    response::Html,
    routing::{get, post},
    Router,
};

use tower_http::{
    services::{ServeDir},
};

// todo: create different svg image states for the tamagotchi state

// todo: update struct to match payload structure

struct CastId {
    fid: u32,
    hash: String,
}

struct UntrustedData {
    fid: u32,
    buttonIndex: u8,
    castId: CastId
}
struct FrameData {
    untrustedData: UntrustedData,
    trustedData: String,
}

async fn initial_frame() -> Html<&'static str> {
    Html("
        <!DOCTYPE html>
        <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\" />
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
                <title>Tamagotchi Frame</title>
                <meta property=\"og:title\" content=\"Tamagotchi Frame\" />
                <meta property=\"og:image\" content=\"https://tamagotch-frame.shuttleapp.rs/tamagotchi.svg\" />
                <meta property=\"fc:frame\" content=\"vNext\" />
                <meta property=\"fc:frame:image\" content=\"https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg\" />
                <meta property=\"fc:frame:button:1\" content=\"Feed\" />
                <meta property=\"fc:frame:button:2\" content=\"Sleep\" />
                <meta property=\"fc:frame:button:3\" content=\"Clean\" />
                <meta property=\"fc:frame:button:4\" content=\"Play\" />
                <meta property=\"fc:frame:post_url\" content=\"https://tamagotch-frame.shuttleapp.rs/api/frame\" />
            </head>
            <body>
                <h1>Tamagotchi Frame</h1>
            </body>
        </html>
    ")
}

async fn handle_button_click(Json(payload): Json<FrameData>) -> Html<&'static str> {
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

    Html("
        <!DOCTYPE html>
        <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\" />
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
                <title>Tamagotchi Frame</title>
                <meta property=\"og:title\" content=\"Tamagotchi Frame\" />
                <meta property=\"og:image\" content=\"https://tamagotch-frame.shuttleapp.rs/tamagotchi.svg\" />
                <meta property=\"fc:frame\" content=\"vNext\" />
                <meta property=\"fc:frame:image\" content=\"https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.svg\" />
                <meta property=\"fc:frame:button:1\" content=\"Feed\" />
                <meta property=\"fc:frame:button:2\" content=\"Sleep\" />
                <meta property=\"fc:frame:button:3\" content=\"Clean\" />
                <meta property=\"fc:frame:button:4\" content=\"Play\" />
                <meta property=\"fc:frame:post_url\" content=\"https://tamagotch-frame.shuttleapp.rs/api/frame\" />
            </head>
            <body>
                <h1>Tamagotchi Frame</h1>
            </body>
        </html>
    ")
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
