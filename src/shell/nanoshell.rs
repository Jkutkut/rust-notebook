use std::io;
use std::io::Write;

use super::shell_handler::ShellHandler;

pub struct Nanoshell<'a> {
    pub title: &'a str,
    pub promt: &'a str,
    pub cmd_handler: ShellHandler,
}

impl Nanoshell<'_> {

    pub fn init(&self) {
        //self.clear_screen(); // TODO remove
        self.print(self.title);
    }

    pub fn run(&self) -> String {
        let mut result: Option<String>;
        loop {
            result = self.handle_cmd(self.get_input());
            match result {
                None => {},
                Some(cmd) => return cmd,
            }
        }
    }

    fn handle_cmd(&self, cmd: String) -> Option<String> {
        match cmd.as_str() {
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
        self.print(self.promt);
        io::stdin()
            .read_line(&mut r)
            .expect("Failed to read line");
        r = r[..r.len() - 1].to_string(); // Remove \n
        r
    }


    // Output

    fn print_stdout(&self, text: &str, flush: bool) {
        print!("{text}");
        if flush {
            io::stdout().flush().unwrap();
        }
    }

    fn print(&self, text: &str) {
        self.print_stdout(text, true);
    }

    fn print_buffered(&self, text: &str) {
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
            "Clears the terminal screen."
        );
        self.help_cmd(
            "help",
            "Shows the help menu."
        );
        self.help_cmd(
            "exit",
            "Exits the program."
        );
        for cmd in self.cmd_handler.cmd_dict.iter() {
            self.help_cmd(&*cmd.name, &*cmd.man);
        }
    }

    fn help_cmd(&self, cmd: &str, man: &str) {
        self.print_buffered(self.promt);
        self.print_buffered(cmd);
        self.print_buffered("\n  ");
        self.print_buffered(man);
        self.print_buffered("\n\n");
    }
}
