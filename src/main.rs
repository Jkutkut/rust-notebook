// extern crate serde;
// extern crate serde_json;
extern crate sqlx;

mod notebook;
use notebook::Notebook;

mod shell;
use shell::shell_handler::ShellHandler;

mod notebook_sqlite;

fn main() {
    let mut nb: Notebook = Notebook::new("sqlite://notebook.db");
    nb.run();
}
