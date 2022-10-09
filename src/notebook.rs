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
        vec![ // TODO update with commands.
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

    pub fn new(file: &'a str) -> Notebook {
        let n = Notebook {
            db: NotebookDB::new(file),
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
    pub fn run(&mut self) {
        self.shell.init();
        let mut cmd: Vec<String>;
        loop {
            cmd = self.shell.run();
            match cmd[0].as_str() {
                "exit" => break,
                "list" => self.list(cmd),
                "add" => self.add(cmd),
                "remove" => self.remove(cmd),
                &_ => todo!(),
            }
        }
        self.shell.print("Exiting notebook\n");
    }

    // Commands

    fn execute_db_cmd(&self, cmd_result: Result<String, String>, cmd: Vec<String>) {
        match cmd_result {
            Ok(msg) => self.shell.print(&msg),
            Err(e) => self.cmd_error(cmd, &e),
        }

    }

    fn list(&self, cmd: Vec<String>) {
        match cmd.len() {
            1 => self.execute_db_cmd(self.db.list_all("all"), cmd),
            2 => self.execute_db_cmd(self.db.list_all(&cmd[1]), cmd),
            3 => self.execute_db_cmd(self.db.list(&cmd[1], &cmd[2]), cmd),
            _ => self.cmd_error(cmd, "Too many arguments"),
        }
    }

    fn add(&mut self, cmd: Vec<String>) {
        match cmd.len() {
            1 => self.cmd_error(cmd, "What do you want to add?"),
            2 => {
                match cmd[1].as_str() {
                    "note" => {
                        let name = self.shell.ask_in_range("  Name: ", 1, 42);
                        let desc = self.shell.ask_in_range("  Description: ", 5, 420);
                        let category = self.shell.ask_or("  Category: ", String::from("UNCATEGORIZED"));
                        let tag = self.shell.ask_or("  Tag: ", String::from("UNTAGGED"));
                        self.execute_db_cmd(self.db.add_note(&name, &desc, &category, &tag), cmd);
                    },
                    "category" | "tag" => {
                        let question = format!("Name of the {}:\n  ", cmd[1]);
                        let ele: String = self.shell.ask_in_range(&question, 1, 420);
                        self.execute_db_cmd(self.db.add(&cmd[1], &ele), cmd);
                    }
                    _ => self.cmd_error(cmd, "Use note, category or tag."),
                }
           },
           3 => self.execute_db_cmd(self.db.add(&cmd[1], &cmd[2]), cmd),
            _ => self.cmd_error(cmd, "Too many arguments"),
        }
    }

    fn remove(&mut self, cmd: Vec<String>) {
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
