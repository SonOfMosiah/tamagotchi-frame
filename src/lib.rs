pub mod types;
pub mod handlers;

use axum::response::IntoResponse;

pub fn generate_svg_with_color(fill_color: &str, option: i64) -> String {
    let pet_svg_path = generate_pet_svg_path(fill_color, option);
    format!("
    <svg id=\"svg\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"400\" height=\"213.33333333333334\" viewBox=\"0, 0, 400,213.33333333333334\">
        <rect width=\"100%\" height=\"100%\" fill=\"#ffffff\" fill-opacity=\"100%\"/>\
        {pet_svg_path}
    </svg>")
}

fn generate_pet_svg_path(fill_color: &str, option: i64) -> String {
    match option {
        1 => generate_mimitchi_svg(fill_color),
        2 => generate_nikatchi_svg(fill_color),
        3 => generate_kutchimatchi_svg(fill_color),
        4 => generate_kutchimatchi_wif_hat_svg(fill_color),
        _ => String::from("Invalid option"),
    }
}

fn generate_mimitchi_svg(fill_color: &str) -> String {
    format!("\
    <g id=\"mimitchi\">\
        <path id=\"path0\" d=\"M146.667 20.000 L 146.667 26.667 140.000 26.667 L 133.333 26.667 133.333 40.000 L 133.333 53.333 140.000 53.333 L 146.667 53.333 146.667 73.333 L 146.667 93.333 140.000 93.333 L 133.333 93.333 133.333 106.667 L 133.333 120.000 126.667 120.000 L 120.000 120.000 120.000 133.333 L 120.000 146.667 126.667 146.667 L 133.333 146.667 133.333 153.333 L 133.333 160.000 140.000 160.000 L 146.667 160.000 146.667 166.667 L 146.667 173.333 153.333 173.333 L 160.000 173.333 160.000 180.000 L 160.000 186.667 153.333 186.667 L 146.667 186.667 146.667 193.333 L 146.667 200.000 166.667 200.000 L 186.667 200.000 186.667 193.333 L 186.667 186.667 220.000 186.667 L 253.333 186.667 253.333 193.333 L 253.333 200.000 260.000 200.000 L 266.667 200.000 266.667 180.000 L 266.667 160.000 273.333 160.000 L 280.000 160.000 280.000 153.333 L 280.000 146.667 286.667 146.667 L 293.333 146.667 293.333 133.333 L 293.333 120.000 286.667 120.000 L 280.000 120.000 280.000 106.667 L 280.000 93.333 273.333 93.333 L 266.667 93.333 266.667 73.333 L 266.667 53.333 273.333 53.333 L 280.000 53.333 280.000 40.000 L 280.000 26.667 273.333 26.667 L 266.667 26.667 266.667 20.000 L 266.667 13.333 253.333 13.333 L 240.000 13.333 240.000 20.000 L 240.000 26.667 233.333 26.667 L 226.667 26.667 226.667 40.000 L 226.667 53.333 220.000 53.333 L 213.333 53.333 213.333 60.000 L 213.333 66.667 206.667 66.667 L 200.000 66.667 200.000 60.000 L 200.000 53.333 193.333 53.333 L 186.667 53.333 186.667 40.000 L 186.667 26.667 180.000 26.667 L 173.333 26.667 173.333 20.000 L 173.333 13.333 160.000 13.333 L 146.667 13.333 146.667 20.000 M253.333 86.667 L 253.333 93.333 260.000 93.333 L 266.667 93.333 266.667 126.667 L 266.667 160.000 246.667 160.000 L 226.667 160.000 226.667 166.667 L 226.667 173.333 193.333 173.333 L 160.000 173.333 160.000 166.667 L 160.000 160.000 153.333 160.000 L 146.667 160.000 146.667 153.333 L 146.667 146.667 153.333 146.667 L 160.000 146.667 160.000 133.333 L 160.000 120.000 153.333 120.000 L 146.667 120.000 146.667 106.667 L 146.667 93.333 153.333 93.333 L 160.000 93.333 160.000 86.667 L 160.000 80.000 206.667 80.000 L 253.333 80.000 253.333 86.667 M160.000 100.000 L 160.000 106.667 166.667 106.667 L 173.333 106.667 173.333 100.000 L 173.333 93.333 166.667 93.333 L 160.000 93.333 160.000 100.000 M240.000 100.000 L 240.000 106.667 246.667 106.667 L 253.333 106.667 253.333 100.000 L 253.333 93.333 246.667 93.333 L 240.000 93.333 240.000 100.000 M186.667 113.333 L 186.667 120.000 206.667 120.000 L 226.667 120.000 226.667 113.333 L 226.667 106.667 206.667 106.667 L 186.667 106.667 186.667 113.333 \" stroke=\"none\" fill=\"{fill_color}\" fill-rule=\"evenodd\"></path>\
    </g>\
    ")
}

fn generate_nikatchi_svg(fill_color: &str) -> String {
    format!("\
    <g id=\"nikatchi\">\
        <path id=\"path0\" d=\"M200.000 20.000 L 200.000 26.667 193.333 26.667 L 186.667 26.667 186.667 33.333 L 186.667 40.000 173.333 40.000 L 160.000 40.000 160.000 46.667 L 160.000 53.333 153.333 53.333 L 146.667 53.333 146.667 60.000 L 146.667 66.667 140.000 66.667 L 133.333 66.667 133.333 100.000 L 133.333 133.333 140.000 133.333 L 146.667 133.333 146.667 140.000 L 146.667 146.667 153.333 146.667 L 160.000 146.667 160.000 153.333 L 160.000 160.000 153.333 160.000 L 146.667 160.000 146.667 173.333 L 146.667 186.667 166.667 186.667 L 186.667 186.667 186.667 180.000 L 186.667 173.333 206.667 173.333 L 226.667 173.333 226.667 180.000 L 226.667 186.667 246.667 186.667 L 266.667 186.667 266.667 173.333 L 266.667 160.000 260.000 160.000 L 253.333 160.000 253.333 153.333 L 253.333 146.667 260.000 146.667 L 266.667 146.667 266.667 140.000 L 266.667 133.333 273.333 133.333 L 280.000 133.333 280.000 100.000 L 280.000 66.667 273.333 66.667 L 266.667 66.667 266.667 60.000 L 266.667 53.333 260.000 53.333 L 253.333 53.333 253.333 46.667 L 253.333 40.000 240.000 40.000 L 226.667 40.000 226.667 33.333 L 226.667 26.667 220.000 26.667 L 213.333 26.667 213.333 20.000 L 213.333 13.333 206.667 13.333 L 200.000 13.333 200.000 20.000 M213.333 40.000 L 213.333 53.333 233.333 53.333 L 253.333 53.333 253.333 60.000 L 253.333 66.667 260.000 66.667 L 266.667 66.667 266.667 100.000 L 266.667 133.333 260.000 133.333 L 253.333 133.333 253.333 140.000 L 253.333 146.667 206.667 146.667 L 160.000 146.667 160.000 140.000 L 160.000 133.333 153.333 133.333 L 146.667 133.333 146.667 100.000 L 146.667 66.667 153.333 66.667 L 160.000 66.667 160.000 60.000 L 160.000 53.333 180.000 53.333 L 200.000 53.333 200.000 40.000 L 200.000 26.667 206.667 26.667 L 213.333 26.667 213.333 40.000 M173.333 73.333 L 173.333 80.000 180.000 80.000 L 186.667 80.000 186.667 73.333 L 186.667 66.667 180.000 66.667 L 173.333 66.667 173.333 73.333 M226.667 73.333 L 226.667 80.000 233.333 80.000 L 240.000 80.000 240.000 73.333 L 240.000 66.667 233.333 66.667 L 226.667 66.667 226.667 73.333 M160.000 106.667 L 160.000 120.000 166.667 120.000 L 173.333 120.000 173.333 126.667 L 173.333 133.333 206.667 133.333 L 240.000 133.333 240.000 126.667 L 240.000 120.000 246.667 120.000 L 253.333 120.000 253.333 106.667 L 253.333 93.333 206.667 93.333 L 160.000 93.333 160.000 106.667 M186.667 113.333 L 186.667 120.000 180.000 120.000 L 173.333 120.000 173.333 113.333 L 173.333 106.667 180.000 106.667 L 186.667 106.667 186.667 113.333 M213.333 113.333 L 213.333 120.000 206.667 120.000 L 200.000 120.000 200.000 113.333 L 200.000 106.667 206.667 106.667 L 213.333 106.667 213.333 113.333 M240.000 113.333 L 240.000 120.000 233.333 120.000 L 226.667 120.000 226.667 113.333 L 226.667 106.667 233.333 106.667 L 240.000 106.667 240.000 113.333 \" stroke=\"none\" fill=\"{fill_color}\" fill-rule=\"evenodd\"></path>
    </g>
    ")
}

fn generate_kutchimatchi_svg(fill_color: &str) -> String {
    format!("\
    <g id=\"kitchimatchi\">\
        <path id=\"path0\" d=\"M186.667 46.667 L 186.667 53.333 180.000 53.333 L 173.333 53.333 173.333 60.000 L 173.333 66.667 153.333 66.667 L 133.333 66.667 133.333 73.333 L 133.333 80.000 126.667 80.000 L 120.000 80.000 120.000 86.667 L 120.000 93.333 126.667 93.333 L 133.333 93.333 133.333 100.000 L 133.333 106.667 126.667 106.667 L 120.000 106.667 120.000 113.333 L 120.000 120.000 126.667 120.000 L 133.333 120.000 133.333 126.667 L 133.333 133.333 146.667 133.333 L 160.000 133.333 160.000 146.667 L 160.000 160.000 166.667 160.000 L 173.333 160.000 173.333 166.667 L 173.333 173.333 180.000 173.333 L 186.667 173.333 186.667 180.000 L 186.667 186.667 220.000 186.667 L 253.333 186.667 253.333 180.000 L 253.333 173.333 260.000 173.333 L 266.667 173.333 266.667 166.667 L 266.667 160.000 273.333 160.000 L 280.000 160.000 280.000 120.000 L 280.000 80.000 273.333 80.000 L 266.667 80.000 266.667 73.333 L 266.667 66.667 260.000 66.667 L 253.333 66.667 253.333 60.000 L 253.333 53.333 246.667 53.333 L 240.000 53.333 240.000 46.667 L 240.000 40.000 213.333 40.000 L 186.667 40.000 186.667 46.667 M240.000 60.000 L 240.000 66.667 246.667 66.667 L 253.333 66.667 253.333 73.333 L 253.333 80.000 260.000 80.000 L 266.667 80.000 266.667 120.000 L 266.667 160.000 260.000 160.000 L 253.333 160.000 253.333 166.667 L 253.333 173.333 220.000 173.333 L 186.667 173.333 186.667 166.667 L 186.667 160.000 180.000 160.000 L 173.333 160.000 173.333 140.000 L 173.333 120.000 153.333 120.000 L 133.333 120.000 133.333 113.333 L 133.333 106.667 153.333 106.667 L 173.333 106.667 173.333 100.000 L 173.333 93.333 153.333 93.333 L 133.333 93.333 133.333 86.667 L 133.333 80.000 153.333 80.000 L 173.333 80.000 173.333 73.333 L 173.333 66.667 180.000 66.667 L 186.667 66.667 186.667 60.000 L 186.667 53.333 213.333 53.333 L 240.000 53.333 240.000 60.000 M186.667 73.333 L 186.667 80.000 193.333 80.000 L 200.000 80.000 200.000 73.333 L 200.000 66.667 193.333 66.667 L 186.667 66.667 186.667 73.333 M226.667 73.333 L 226.667 80.000 233.333 80.000 L 240.000 80.000 240.000 73.333 L 240.000 66.667 233.333 66.667 L 226.667 66.667 226.667 73.333 \" stroke=\"none\" fill=\"{fill_color}\" fill-rule=\"evenodd\"></path>
    </g>
    ")
}

fn generate_kutchimatchi_wif_hat_svg(fill_color: &str) -> String {
    format!("\
    <g id=\"kitchimatchi_wif_hat\">\
        <path id=\"path0\" d=\"M190.476 47.619 L 190.476 66.667 180.952 66.667 L 171.429 66.667 171.429 71.429 L 171.429 76.190 180.952 76.190 L 190.476 76.190 190.476 80.952 L 190.476 85.714 185.714 85.714 L 180.952 85.714 180.952 90.476 L 180.952 95.238 166.667 95.238 L 152.381 95.238 152.381 100.000 L 152.381 104.762 147.619 104.762 L 142.857 104.762 142.857 109.524 L 142.857 114.286 147.619 114.286 L 152.381 114.286 152.381 119.048 L 152.381 123.810 147.619 123.810 L 142.857 123.810 142.857 128.571 L 142.857 133.333 147.619 133.333 L 152.381 133.333 152.381 138.095 L 152.381 142.857 161.905 142.857 L 171.429 142.857 171.429 152.381 L 171.429 161.905 176.190 161.905 L 180.952 161.905 180.952 166.667 L 180.952 171.429 185.714 171.429 L 190.476 171.429 190.476 176.190 L 190.476 180.952 214.286 180.952 L 238.095 180.952 238.095 176.190 L 238.095 171.429 242.857 171.429 L 247.619 171.429 247.619 166.667 L 247.619 161.905 252.381 161.905 L 257.143 161.905 257.143 133.333 L 257.143 104.762 252.381 104.762 L 247.619 104.762 247.619 100.000 L 247.619 95.238 242.857 95.238 L 238.095 95.238 238.095 90.476 L 238.095 85.714 233.333 85.714 L 228.571 85.714 228.571 80.952 L 228.571 76.190 238.095 76.190 L 247.619 76.190 247.619 71.429 L 247.619 66.667 238.095 66.667 L 228.571 66.667 228.571 47.619 L 228.571 28.571 209.524 28.571 L 190.476 28.571 190.476 47.619 M219.048 52.381 L 219.048 66.667 209.524 66.667 L 200.000 66.667 200.000 52.381 L 200.000 38.095 209.524 38.095 L 219.048 38.095 219.048 52.381 M228.571 90.476 L 228.571 95.238 233.333 95.238 L 238.095 95.238 238.095 100.000 L 238.095 104.762 242.857 104.762 L 247.619 104.762 247.619 133.333 L 247.619 161.905 242.857 161.905 L 238.095 161.905 238.095 166.667 L 238.095 171.429 214.286 171.429 L 190.476 171.429 190.476 166.667 L 190.476 161.905 185.714 161.905 L 180.952 161.905 180.952 147.619 L 180.952 133.333 166.667 133.333 L 152.381 133.333 152.381 128.571 L 152.381 123.810 166.667 123.810 L 180.952 123.810 180.952 119.048 L 180.952 114.286 166.667 114.286 L 152.381 114.286 152.381 109.524 L 152.381 104.762 166.667 104.762 L 180.952 104.762 180.952 100.000 L 180.952 95.238 185.714 95.238 L 190.476 95.238 190.476 90.476 L 190.476 85.714 209.524 85.714 L 228.571 85.714 228.571 90.476 M190.476 100.000 L 190.476 104.762 195.238 104.762 L 200.000 104.762 200.000 100.000 L 200.000 95.238 195.238 95.238 L 190.476 95.238 190.476 100.000 M219.048 100.000 L 219.048 104.762 223.810 104.762 L 228.571 104.762 228.571 100.000 L 228.571 95.238 223.810 95.238 L 219.048 95.238 219.048 100.000 \" stroke=\"none\" fill=\"{fill_color}\" fill-rule=\"evenodd\"></path>\
    </g>\
    ")
}