pub struct FtDictEntry {
    pub name: String,
    pub cmd: String,
    pub man: String,
}

impl FtDictEntry {
    pub fn new(name: &str, cmd: &str, man: &str) -> Self {
        FtDictEntry {
            name: String::from(name),
            cmd: String::from(cmd),
            man: String::from(man),
        }
    }
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
