mod command;

use command::{ init, Command };
use std::io::{ self };

fn main() {
    fn out(data: String) {
        println!("{}", data)
    }

    let reg = init(out);

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match Command::from(&input) {
                    Ok(cmd) => {
                        reg.exec(cmd.name, cmd.args);
                    }
                    Err(err) => {
                        eprintln!("Parse Error: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}
