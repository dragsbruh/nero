# Command Line Interface

## About

This is a simple command line interface edition of Nero. It acts similar to cmd.exe except with Nero commands.
Replace contents of src/main.rs with this code (v0.3.1) for a simple command line interface edition of Nero.

## Source Code

```rust

#![allow(dead_code)]
#![allow(unused_macros)] // Future contributors, remove this after beta
mod commands;
mod utils;
mod core;
mod nero;

use crate::core::Output;
use nero::Nero;
use std::io;

fn main() {
    fn out(output: Output) {
        match output {
            Output::Media(media) => {
                println("Received media data for {}. Ignoring.". media.name);
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
```
