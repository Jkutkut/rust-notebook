use crate::shell::shell_handler::FtDictEntry;

pub fn notebook_cmds() -> Box<[FtDictEntry]> {
    Box::new([
        FtDictEntry {
            name: String::from("list"),
            ft: do_nothing,
        },
        FtDictEntry {
            name: String::from("add"),
            ft: do_nothing,
        },
        FtDictEntry {
            name: String::from("remove"),
            ft: do_nothing,
        },
        FtDictEntry {
            name: String::from("help"),
            ft: do_nothing,
        },
    ])
}


// Functions

pub fn do_nothing(cmd: String) {
    print!("Doing nothing... cmd: '{cmd}'\n");
}
