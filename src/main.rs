use std::collections::HashMap;

mod notebook;
use notebook::Notebook;

mod notebook_json;

mod shell;
use shell::nanoshell::Nanoshell;
use shell::shell_handler::ShellHandler;

fn main() {
    let mut nb: Notebook = Notebook {
        file: "notebook.json",
        shell: Nanoshell{
            title: "Rust-Notebook\n\n",
            notes: HashMap::new();
            promt: "$> ",
            cmd_handler: ShellHandler{
                cmd_dict: notebook::notebook_cmds(),
            },
        },
    };
    nb.init();
    nb.run();
}
