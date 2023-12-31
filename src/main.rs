#![allow(dead_code)]
#![allow(unused_macros)] // Future contributors, remove this after beta
mod commands;
mod utils;
mod data;
mod core;
mod nero;

use crate::core::Output;
use crate::core::Text; // Add import for Media here if you get error in helper macros in commands.rs
use crate::nero::Nero;
use std::fs::File;
use base64::decode;
use std::io::{ self, Write };

fn main() {
    fn out(output: Output) {
        match output {
            Output::Media(media) => {
                let raw_data = match decode(media.data) {
                    Ok(data) => data,
                    Err(err) => {
                        text_output!(out, format!("Media decode error: {}", err));
                        return;
                    }
                };
                let mut file = match File::create(media.name) {
                    Ok(file) => file,
                    Err(err) => {
                        text_output!(out, format!("Error creating file: {}", err));
                        return;
                    }
                };
                match file.write_all(&raw_data) {
                    Ok(_) => {}
                    Err(err) => {
                        text_output!(out, format!("Error writing to file: {}", err));
                    }
                }
            }
            Output::Text(text) => {
                println!("{}", text.data);
            }
        }
    }

    let nero = Nero::new(out);

    loop {
        let mut user_input = String::new();

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                match nero.exec(user_input.clone()) {
                    Ok(_) => {}
                    Err(err) => text_output!(out, format!("Failed to execute: {}", err)),
                }
            }
            Err(err) => {
                text_output!(out, format!("Failed to read input: {}", err));
            }
        }
    }
}
