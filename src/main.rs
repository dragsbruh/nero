#![allow(dead_code)]
#![allow(unused_macros)] // Future contributors, remove this after beta
mod command;
mod nero;

use command::Output;
use nero::Nero;

use std::io;

fn main() {
    fn out(output: Output) {
        match output {
            Output::Media(media) => {
                println!("Media saved to: {}", media.name);
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
                    Err(err) => eprintln!("Failed to execute: {}", err),
                }
            }
            Err(err) => {
                eprintln!("Failed to read input: {}", err);
            }
        }
    }
}
