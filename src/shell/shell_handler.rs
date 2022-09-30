pub struct FtDictEntry {
    pub name: String,
    pub man: String,
}

pub struct ShellHandler {
    pub cmd_dict: Vec<FtDictEntry>,
}

impl ShellHandler {
    pub fn is_cmd(&self, cmd: &String) -> bool {
        for f in self.cmd_dict.iter() {
            if f.name.eq(cmd) {
                return true;
            }
        }
        return false;
    }
}
