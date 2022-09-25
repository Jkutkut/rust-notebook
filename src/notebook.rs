use crate::shell::shell_handler::FtDictEntry;

pub fn notebook_cmds() -> Box<[FtDictEntry]> {
    Box::new([
        FtDictEntry {
            name: String::from("list"),
            ft: do_nothing,
            man: String::from("Show all the elements."),
        },
        FtDictEntry {
            name: String::from("add"),
            ft: do_nothing,
            man: String::from("Opens a form to fill all the data."),
        },
        FtDictEntry {
            name: String::from("remove"),
            ft: do_nothing,
            man: String::from("Removes a element by name."),
        },
    ])
}


// Functions

pub fn do_nothing(cmd: String) {
    print!("Doing nothing... cmd: '{cmd}'\n");
}



