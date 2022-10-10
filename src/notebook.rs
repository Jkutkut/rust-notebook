use crate::shell::shell_handler::ShellHandler;
use crate::shell::shell_handler::FtDictEntry;
use crate::shell::nanoshell::Nanoshell;
use crate::notebook_sqlite::NotebookDB;

// Notebook
pub struct Notebook {
        db: NotebookDB,
    shell: Nanoshell,
}

// Constructor
impl Notebook {
    fn cmd_dict() -> Vec<FtDictEntry> {
        vec![
            FtDictEntry::new(
                "list all",
                "list",
                "List all the notes."
            ),
            FtDictEntry::new(
                "list all sorted by",
                "list <category|tag>",
                "Shows all the notes by category or by tag."
            ),
            FtDictEntry::new(
                "list categories or tags",
                "list <categories|tags>",
                "Shows all the categories or tags."
            ),
            FtDictEntry::new(
                "list specific category or tag",
                "list <category|tag> <CATEGORY|TAG>",
                "Shows all the elements on the given category or tag."
            ),
            FtDictEntry::new(
                "add",
                "add <note|category|tag>",
                "Opens a form to add the selected element."
            ),
            FtDictEntry::new(
                "add category or tag directly",
                "add <category|tag> <CATEGORY|TAG>",
                "Adds the selected category or tag directly."
            ),
            FtDictEntry::new(
                "remove",
                "remove <note|category|tag>",
                "Removes the selected element by it's name."
            ),
            FtDictEntry::new(
                "remove note, category or tag directly",
                "remove <note|category|tag> <NAME>",
                "Removes the selected element."
            )
        ]
    }

    pub fn new(file: &str) -> Notebook {
        let n = Notebook {
            db: NotebookDB::new(file),
            shell: Nanoshell::new(
                String::from("Rust-Notebook\n\n"),
                ShellHandler {
                    cmd_dict: Notebook::cmd_dict()
                }
            )
        };
        n
    }
}

// Methods
impl Notebook {
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
            Ok(msg) => {
                self.shell.print_buffered(&msg);
                self.shell.print("\n");
            },
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
        match cmd.len() {
            0 | 1 => self.cmd_error(cmd, "What do you want to remove?"),
            2 => {
                let ele = self.shell.ask("  Name: ");
                self.execute_db_cmd(self.db.remove(&cmd[1], &ele), cmd);
            }
            3 => self.execute_db_cmd(self.db.remove(&cmd[1], &cmd[2]), cmd),
            _ => self.cmd_error(cmd, "Too many arguments"),
        }
    }

    // Error

    fn cmd_error(&self, cmd: Vec<String>, error: &str) {
        self.shell.print_buffered("Error in ");
        self.shell.print_buffered(cmd[0].as_str());
        self.shell.print_buffered(": ");
        self.shell.print_buffered(error);
        self.shell.print("\n");
    }
}
