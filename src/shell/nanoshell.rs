use std::io;
use std::io::Write;

pub struct Nanoshell<'a> {
    pub title: &'a str,
    pub promt: &'a str,
}

impl Nanoshell<'_> {

    pub fn run(&self) {
        self.print(self.title);

        let mut input: String;
        loop {
            input = self.get_input();
            match input.as_str() {
                "exit\n" => break,
                _ => self.print("Command not found\n"),
            }
        }

        self.print("Exiting Shell\n");
    }

    fn get_input(&self) -> String {
        // TODO remove \n
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
