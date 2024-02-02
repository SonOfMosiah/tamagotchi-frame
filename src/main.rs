use axum::{
    response::Html,
    routing::{get, post},
    Router,
};

use tower_http::{
    services::{ServeDir, ServeFile},
};

// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // todo: turn each route into a function
    let router = Router::new()
        .route("/", get( Html("
            <!DOCTYPE html>
            <html lang=\"en\">
                <head>
                    <meta charset=\"UTF-8\" />
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
                    <title>Tamagotchi Frame</title>
                    <meta property=\"og:title\" content=\"Tamagotchi Frame\" />
                    <meta property=\"og:image\" content=\"https://tamagotch-frame.shuttleapp.rs/tamagotchi.png\" />
                    <meta property=\"fc:frame\" content=\"vNext\" />
                    <meta property=\"fc:frame:image\" content=\"https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.png\" />
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
        ")))
        .route("/api/frame", post(Html("
            <!DOCTYPE html>
            <html lang=\"en\">
                <head>
                    <meta property=\"fc:frame\" content=\"vNext\" />
                    <meta property=\"fc:frame:image\" content=\"https://tamagotch-frame.shuttleapp.rs/public/tamagotchi.png\" />
                    <meta property=\"fc:frame:button:1\" content=\"Feed\" />
                    <meta property=\"fc:frame:button:2\" content=\"Sleep\" />
                    <meta property=\"fc:frame:button:3\" content=\"Clean\" />
                    <meta property=\"fc:frame:button:4\" content=\"Play\" />
                    <meta property=\"fc:frame:post_url\" content=\"https://tamagotchi-frame.fly.dev/api/frame\" />
                </head>
            </html>
        "))).nest_service("/public", ServeDir::new("public"))
        .route("/tamagotchi.png", get(ServeFile::new("public/tamagotchi.png")));

    Ok(router.into())
}
