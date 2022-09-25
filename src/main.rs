use std::collections::HashMap;

mod notebook;
use notebook::Notebook;

mod shell;
use shell::nanoshell::Nanoshell;
use shell::shell_handler::ShellHandler;

fn main() {
    let note_book: Notebook = Notebook {
        notes: HashMap::new(),
    };

    let s = Nanoshell{
        title: "Rust-Notebook\n\n",
        promt: "$> ",
        cmd_handler: ShellHandler{
            // cmd_dict: Box::new([]),
            cmd_dict: notebook::notebook_cmds(),
        },
    };

    s.run();
}
