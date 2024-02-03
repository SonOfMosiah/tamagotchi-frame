use axum::extract::{Path, State};
use axum::http::{StatusCode, Request};
use axum::Json;
use axum::response::{Html, IntoResponse};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::generate_svg_with_color;
use crate::types::{FrameData, TamagotchiId};

use entity::Entity as Tamagotchi;

const TAMAGOTCHI_PET_OPTIONS: i64 = 3;

// todo: implement actions (feed, light, duck, health meter, play, medicine, attention, discipline)
// todo: 3 buttons A, B, C (select, confirm, menu/cancel)

// todo: 8 icons (feed, light, duck, health meter, play, medicine, attention, discipline)



/// Generates HTML response for the frame with a dynamic image, dynamic buttons, and dynamic post URL.
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

// todo: create different svg image states for the tamagotchi state
pub async fn get_tamagotchi(Path(TamagotchiId { fid }): Path<TamagotchiId>) -> impl IntoResponse {
    // get the tamagotchi options based on fid
    let (option, color) = get_tamagotchi_options(fid);

    println!("Option: {}", option);
    println!("Color: {}", color);

    let tamagotchi_image = generate_svg_with_color(&color, option);
    (StatusCode::OK, [("Content-Type", "image/svg+xml")], tamagotchi_image)
}

fn get_tamagotchi_options(fid: i64) -> (i64, String) {
    let mut option = fid % TAMAGOTCHI_PET_OPTIONS + 1;
    if option == 3 {
        if fid < 2000 {
            option = 4; // the rich get richer
        }
    }

    // Hash the fid to get a consistent, random-seeming value
    let mut hasher = Sha256::new();
    hasher.update(fid.to_string().as_bytes());
    let result = hasher.finalize();

    // Use parts of the hash for the color components
    let color = format!(
        "#{:02x}{:02x}{:02x}",
        result[0] % 128, // Red component
        result[1] % 128, // Green component
        result[2] % 128  // Blue component
    );

    (option, color)
}
pub async fn initial_frame() -> Html<String> {
    // // Extract the Host header from the request
    // let host = req.headers().get("Host").and_then(|v| v.to_str().ok()).unwrap_or("");
    //
    // // Assuming HTTP for simplicity; you might need to adjust based on your deployment (e.g., HTTPS)
    // let base_url = format!("http://{}", host);

    let base_url = "https://tamagotch-frame.shuttleapp.rs";

    let image_url = format!("{base_url}/public/start_page.png");

    let button_names = ["Enter"];
    let post_url = "https://tamagotch-frame.shuttleapp.rs/api/connect";
    generate_html_response(&image_url, &button_names, post_url).await
}

pub async fn connect_tamagotchi(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<FrameData>
) -> Result<Html<String>, String> {
    let fid = payload.get_fid();
    let (option, color) = get_tamagotchi_options(fid);

    // Attempt to find the existing Tamagotchi
    let tamagotchi_result = Tamagotchi::find_by_id(fid).one(&db).await;

    println!("after find_by_id");

    // Handle potential database errors
    let tamagotchi = match tamagotchi_result {
        Ok(tamagotchi) => tamagotchi,
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    if tamagotchi.is_none() {
        println!("Tamagotchi does not exist yet");

        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|_| "Time went backwards")?;
        let seconds = now.as_secs();

        let new_tamagotchi = entity::ActiveModel{
            fid: ActiveValue::set(fid),
            color: ActiveValue::set(color),
            option: ActiveValue::set(option as i8),
            created_at: ActiveValue::set(seconds), // Assuming `created_at` is i64
            last_interaction: ActiveValue::set(seconds),
            health: ActiveValue::set(100),
            hunger: ActiveValue::set(100),
            sleepiness: ActiveValue::set(100),
            dirtiness: ActiveValue::set(100),
            happiness: ActiveValue::set(100),
            version: ActiveValue::set(1),
        };

        // Insert and handle potential database errors
        Tamagotchi::insert(new_tamagotchi).exec(&db).await
            .map_err(|e| format!("Insertion error: {}", e))?;
    } else {
        println!("Tamagotchi already exists");
    }

    let image_url = format!("https://tamagotch-frame.shuttleapp.rs/api/tamagotchi/{fid}");
    let button_names = ["Feed", "Sleep", "Clean", "Play"];
    let post_url = "https://tamagotch-frame.shuttleapp.rs/api/actions";

    Ok(generate_html_response(&image_url, &button_names, &post_url).await)
}

pub async fn handle_action_click(State(db): State<DatabaseConnection>, Json(payload): Json<FrameData>) -> Result<Html<String>, String> {
    // todo: validate message
    println!("Received action click: {:?}", payload);

    let button_index = payload.get_button_index();
    let fid = payload.get_fid();
    println!("Button index: {}", button_index);
    println!("Fid: {}", fid);

    let tamagotchi_result = Tamagotchi::find_by_id(fid).one(&db).await;

    let tamagotchi = match tamagotchi_result {
        Ok(Some(model)) => model,
        Ok(None) => return Err("Tamagotchi not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    let mut active_tamagotchi: entity::ActiveModel = tamagotchi.clone().into();

    // todo: update the tamagotchi state in the db
    match button_index {
        1 => {
            let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?
                .as_secs() - tamagotchi.last_interaction;

            let elapsed_days = elapsed_time / 86400;
            let attribute_decrease = elapsed_days as i8 * 100;

            let new_hunger = if tamagotchi.hunger - attribute_decrease + 20 > 100 { 100 } else { tamagotchi.hunger - attribute_decrease + 20 };
            let new_sleepiness = tamagotchi.sleepiness - attribute_decrease;
            let new_dirtiness = tamagotchi.dirtiness - attribute_decrease;
            let new_happiness = tamagotchi.dirtiness - attribute_decrease;

            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?;
            let seconds = now.as_secs();

            active_tamagotchi.hunger = ActiveValue::set(new_hunger);
            active_tamagotchi.sleepiness = ActiveValue::set(new_sleepiness);
            active_tamagotchi.dirtiness = ActiveValue::set(new_dirtiness);
            active_tamagotchi.happiness = ActiveValue::set(new_happiness);
            active_tamagotchi.last_interaction = ActiveValue::Set(seconds);
        }
        2 => {
            let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?
                .as_secs() - tamagotchi.last_interaction;

            let elapsed_days = elapsed_time / 86400;
            let attribute_decrease = elapsed_days as i8 * 100;

            let new_hunger = tamagotchi.hunger - attribute_decrease;
            let new_sleepiness = if tamagotchi.sleepiness - attribute_decrease + 20 > 100 { 100 } else { tamagotchi.sleepiness - attribute_decrease + 20 };
            let new_dirtiness = tamagotchi.dirtiness - attribute_decrease;
            let new_happiness = tamagotchi.happiness - attribute_decrease;

            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?;
            let seconds = now.as_secs();

            active_tamagotchi.hunger = ActiveValue::set(new_hunger);
            active_tamagotchi.sleepiness = ActiveValue::set(new_sleepiness);
            active_tamagotchi.dirtiness = ActiveValue::set(new_dirtiness);
            active_tamagotchi.happiness = ActiveValue::set(new_happiness);
            active_tamagotchi.last_interaction = ActiveValue::Set(seconds);
        }
        3 => {
            let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?
                .as_secs() - tamagotchi.last_interaction;

            let elapsed_days = elapsed_time / 86400;
            let attribute_decrease = elapsed_days as i8 * 100;

            let new_hunger = tamagotchi.hunger - attribute_decrease;
            let new_sleepiness = tamagotchi.sleepiness - attribute_decrease;
            let new_dirtiness = if tamagotchi.dirtiness - attribute_decrease + 20 > 100 { 100 } else { tamagotchi.dirtiness - attribute_decrease + 20 };
            let new_happiness = tamagotchi.happiness - attribute_decrease;

            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?;
            let seconds = now.as_secs();

            active_tamagotchi.hunger = ActiveValue::set(new_hunger);
            active_tamagotchi.sleepiness = ActiveValue::set(new_sleepiness);
            active_tamagotchi.dirtiness = ActiveValue::set(new_dirtiness);
            active_tamagotchi.happiness = ActiveValue::set(new_happiness);
            active_tamagotchi.last_interaction = ActiveValue::Set(seconds);
        }
        4 => {
            // todo: send user to the game page (guess the number first)
            let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?
                .as_secs() - tamagotchi.last_interaction;

            let elapsed_days = elapsed_time / 86400;
            let attribute_decrease = elapsed_days as i8 * 100;

            let new_hunger = tamagotchi.hunger - attribute_decrease;
            let new_sleepiness = tamagotchi.sleepiness - attribute_decrease;
            let new_dirtiness = tamagotchi.dirtiness - attribute_decrease;
            let new_happiness = if tamagotchi.happiness - attribute_decrease + 20 > 100 { 100 } else { tamagotchi.happiness - attribute_decrease + 20 };

            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|_| "Time went backwards")?;
            let seconds = now.as_secs();

            active_tamagotchi.hunger = ActiveValue::set(new_hunger);
            active_tamagotchi.sleepiness = ActiveValue::set(new_sleepiness);
            active_tamagotchi.dirtiness = ActiveValue::set(new_dirtiness);
            active_tamagotchi.happiness = ActiveValue::set(new_happiness);
            active_tamagotchi.last_interaction = ActiveValue::Set(seconds);
        }
        _ => {}
    }

    let image_url = format!("https://tamagotch-frame.shuttleapp.rs/api/tamagotchi/{fid}");
    let button_names = ["Next Action"];
    let post_url = "https://tamagotch-frame.shuttleapp.rs/api/connect";
    Ok(generate_html_response(&image_url, &button_names, &post_url).await)
}