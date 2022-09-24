mod shell;


fn main() {
    let s = shell::nanoshell::Nanoshell{
        title: "Rust-Notebook\n\n",
        promt: "$>",
    };

    s.run();
}
