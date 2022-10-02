extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

mod notebook;
use notebook::Notebook;

mod notebook_json;

mod shell;
use shell::shell_handler::ShellHandler;

fn main() {
    let mut nb: Notebook = Notebook::new("notebook.json");
    nb.run();
}
