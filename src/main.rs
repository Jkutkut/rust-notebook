use std::collections::HashMap;

mod notebook;
use notebook::Notebook;

mod shell;
use shell::nanoshell::Nanoshell;
use shell::shell_handler::ShellHandler;

fn main() {
    let nb: Notebook = Notebook {
        notes: HashMap::new(),
        shell: Nanoshell{
            title: "Rust-Notebook\n\n",
            promt: "$> ",
            cmd_handler: ShellHandler{
                cmd_dict: notebook::notebook_cmds(),
            },
        },
    };
    nb.run();
}
