use crate::shell::shell_handler::ShellHandler;
use crate::shell::shell_handler::FtDictEntry;
use crate::shell::nanoshell::Nanoshell;
use crate::notebook_sqlite::NotebookDB;

// Notebook
pub struct Notebook<'a> {
    db: NotebookDB,   
    shell: Nanoshell<'a>,
}

// Constructor
impl<'a> Notebook<'a> {
    fn cmd_dict() -> Vec<FtDictEntry> {
        vec![
            FtDictEntry {
                name: String::from("list"),
                cmd: String::from("list <category|tag> [CATEGORY|TAG]"),
                man: String::from("Shows all the notes by categories or by tags."),
            },
            FtDictEntry {
                name: String::from("add"),
                cmd: String::from("add <note|category|tag>"),
                man: String::from("Adds the selected element."),
            },
            FtDictEntry {
                name: String::from("remove"),
                cmd: String::from("remove <note|category|tag>"),
                man: String::from("Removes the selected element."),
            },
        ]
    }

    pub async fn new(file: &'a str) -> Notebook {
        let n = Notebook {
            db: NotebookDB::new(file).await,
            shell: Nanoshell {
                title: "Rust-Notebook\n\n",
                promt: "\x1b[38;5;33m$>\x1b[0m ",
                cmd_handler: ShellHandler {
                    cmd_dict: Notebook::cmd_dict(),
                },
            },
        };
        n
    }
}

// Methods
impl Notebook<'_> {
    pub async fn run(&mut self) {
        self.shell.init();
        let mut cmd: Vec<String>;
        loop {
            cmd = self.shell.run();
            match cmd[0].as_str() {
                "exit" => break,
                "list" => self.list(cmd).await,
                "add" => self.add(cmd).await,
                "remove" => self.remove(cmd).await,
                &_ => todo!(),
            }
        }
        self.shell.print("Exiting notebook\n");
    }

    // Commands

    async fn execute_db_cmd(&self, cmd_result: Result<&str, &str>, cmd: Vec<String>) {
        match cmd_result {
            Ok(msg) => self.shell.print(msg),
            Err(e) => self.cmd_error(cmd, e),
        }

    }

    async fn list(&self, cmd: Vec<String>) {
        match cmd.len() {
            0 | 1 => self.cmd_error(cmd, "Enter the type desired"),
            2 => self.execute_db_cmd(self.db.list(&cmd[1]).await, cmd).await,
            3 => self.cmd_error(cmd, "Not implemented yet"),
            _ => self.cmd_error(cmd, "Too many arguments"),
        }
    }

    async fn add(&mut self, cmd: Vec<String>) {
        // TODO implement with DB
        return self.cmd_error(cmd, "Not implemented yet");
    }

    async fn remove(&mut self, cmd: Vec<String>) {
        // TODO implement with DB
        return self.cmd_error(cmd, "Not implemented yet");
    }

    // Error

    fn cmd_error(&self, cmd: Vec<String>, error: &str) {
        self.shell.print_buffered(cmd[0].as_str());
        self.shell.print_buffered(": Error: ");
        self.shell.print_buffered(error);
        self.shell.print("\n\n");
    }
}
