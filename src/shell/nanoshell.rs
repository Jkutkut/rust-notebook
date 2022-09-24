use std::io;
use std::io::Write;

pub struct Nanoshell<'a> {
    pub title: &'a str,
    pub promt: &'a str,
}

impl Nanoshell<'_> {

    pub fn run(&self) {
        self.print(self.title);
        self.print(&self.get_input());
    }

    fn get_input(&self) -> String {
        let mut r = String::new();
        self.print(self.promt);
        io::stdin()
            .read_line(&mut r)
            .expect("Failed to read line");
        r
    }

    fn print(&self, text: &str) {
        //std::io::stdout().write(text);
        print!("{text}");
        io::stdout().flush().unwrap();
    }
}
