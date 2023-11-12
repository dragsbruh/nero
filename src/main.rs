mod command;
mod nero;

use nero::Nero;
use std::io;

fn main() {
    fn out(output: String) {
        println!("{}", output);
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
