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
                "exit" => break,
                "clear" => self.clear_screen(),
                "" => {},
                _ => self.print("Command not found\n"),
            }
        }

        self.print("Exiting Shell\n");
    }

    fn get_input(&self) -> String {
        let mut r = String::new();
        self.print(self.promt);
        io::stdin()
            .read_line(&mut r)
            .expect("Failed to read line");
        r = r[..r.len() - 1].to_string(); // Remove \n
        r
    }

    fn print(&self, text: &str) {
        print!("{text}");
        io::stdout().flush().unwrap();
    }

    fn clear_screen(&self) {
        let esc_char: char = 27 as char;
        // 033[H -> Set cursor on home position.
        // 033[2J -> Clear screen.
        print!("{esc_char}[H{esc_char}[2J");
    }
}
