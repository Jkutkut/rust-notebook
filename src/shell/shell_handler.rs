pub struct FtDictEntry {
    name: String,
    ft: fn(cmd: String),
}



pub struct ShellHandler {
    pub cmd_dict: Box<[FtDictEntry]>,
}

impl ShellHandler {
    pub fn handle(&self, cmd: String) -> bool {
        for f in self.cmd_dict.iter() {
            if f.name.eq(&cmd) {
                (f.ft)(cmd);
                return true;
            }
        }
        return false;
    }
}
