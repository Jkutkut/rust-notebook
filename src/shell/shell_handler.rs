pub struct FtDictEntry {
    pub name: String,
    // pub cmd: String, // TODO implement
    pub man: String,
}

pub struct ShellHandler {
    pub cmd_dict: Vec<FtDictEntry>,
}

impl ShellHandler {
    pub fn is_cmd(&self, cmd: &Vec<String>) -> bool {
        for f in self.cmd_dict.iter() {
            if f.name.eq(&cmd[0]) {
                return true;
            }
        }
        return false;
    }
}
