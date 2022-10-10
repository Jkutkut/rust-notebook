use std::io;
use std::io::Write;

use super::shell_handler::ShellHandler;
use super::colors;

pub struct Nanoshell {
    pub title: String,
    pub promt: String,
    pub cmd_handler: ShellHandler,
}

impl Nanoshell {

    pub fn new(title: String, cmd_handler: ShellHandler) -> Self {
        let n = Nanoshell {
            title: title,
            promt: format!("{}$>{} ", colors::BLUE, colors::NC),
            cmd_handler: cmd_handler
        };
        n
    }

    pub fn init(&self) {
        //self.clear_screen();
        self.print(&self.title);
    }

    pub fn run(&self) -> Vec<String> {
        let mut result: Option<Vec<String>>;
        loop {
            result = self.handle_cmd(self.get_cmd());
            match result {
                None => {},
                Some(cmd) => return cmd,
            }
        }
    }

    fn handle_cmd(&self, cmd: Vec<String>) -> Option<Vec<String>> {
        match cmd[0].as_str() {
            "" => {},
            "exit" => return Some(cmd),
            "clear" => self.clear_screen(),
            "help" => self.help(),
            _ => {
                if self.cmd_handler.is_cmd(&cmd) {
                    return Some(cmd);
                }
                else {
                    self.cmd_not_found();
                }
            },
        }
        return None;
    }

    fn cmd_not_found(&self) {
        self.print("Command not found\n");
    }

    // Input

    fn get_input(&self) -> String {
        let mut r = String::new();
        self.print(&self.promt);
        io::stdin()
            .read_line(&mut r)
            .expect("Failed to read line");
        r = r[..r.len() - 1].trim().to_string(); // Remove \n and trim it
        r
    }

    fn get_cmd(&self) -> Vec<String> {
        let r: String = self.get_input();
        let split: Vec<&str> = r.split(" ").collect();
        let mut input: Vec<String> = Vec::<String>::new();
        for s in split {
            input.push(String::from(s));
        }
        input
    }

    pub fn ask(&self, question: &str) -> String {
        self.print(question);
        self.get_input()
    }

    pub fn ask_in_range(&self, question: &str, min: usize, max: usize) -> String {
        let mut r: String;
        let error_len = format!("The response should have length in range [{min}, {max}]\n");
        loop {
            r = self.ask(question);
            if r.len() >= min && r.len() <= max {
                return r;
            }
            self.print(&error_len);
        }
    }

    pub fn ask_or(&self, question: &str, default: String) -> String {
        let r = self.ask(question);
        match r.len() {
            0 => default,
            _ => r,
        }
    }

    // Output

    fn print_stdout(&self, text: &str, flush: bool) {
        print!("{text}");
        if flush {
            io::stdout().flush().unwrap();
        }
    }

    pub fn print(&self, text: &str) {
        self.print_stdout(text, true);
    }

    pub fn print_buffered(&self, text: &str) {
        self.print_stdout(text, false);
    }

    // Commands

    fn clear_screen(&self) {
        let esc_char: char = 27 as char;
        // 033[H -> Set cursor on home position.
        // 033[2J -> Clear screen.
        print!("{esc_char}[H{esc_char}[2J");
    }

    fn help(&self) {
        self.clear_screen();
        self.print_buffered("Help manual:\n\n");
        self.help_cmd(
            "clear",
            "clear",
            "Clears the terminal screen."
        );
        self.help_cmd(
            "help",
            "help",
            "Shows the help menu."
        );
        self.help_cmd(
            "exit",
            "exit",
            "Exits the program."
        );
        for cmd in self.cmd_handler.cmd_dict.iter() {
            self.help_cmd(&*cmd.name, &*cmd.cmd, &*cmd.man);
        }
    }

    fn help_cmd(&self, name: &str, cmd: &str, man: &str) {
        self.print_buffered(colors::YELLOW);
        self.print_buffered(name);
        self.print_buffered(colors::NC);
        self.print_buffered("\n  ");
        self.print_buffered(&self.promt);
        self.print_buffered(cmd);
        self.print_buffered("\n  ");
        self.print_buffered(man);
        self.print_buffered("\n\n");
    }
}
