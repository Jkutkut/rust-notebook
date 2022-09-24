mod shell;
use shell::nanoshell::Nanoshell;

mod notebook;
use shell::shell_handler::ShellHandler;

fn main() {
    let s = Nanoshell{
        title: "Rust-Notebook\n\n",
        promt: "$> ",
        cmd_handler: ShellHandler{},
    };

    s.run();
}
