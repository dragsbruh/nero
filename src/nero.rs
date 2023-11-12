use crate::command::{ Registry, OutFun, Command, init };

pub struct Nero {
    pub reg: Registry,
}
impl Nero {
    pub fn new(out: OutFun) -> Nero {
        let reg = init(out);
        Self {
            reg,
        }
    }
    pub fn exec(&self, cmd: String) -> Result<(), String> {
        match Command::new(&cmd) {
            Ok(cmd) => {
                self.reg.exec(cmd.name, cmd.args);
                return Ok(());
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
