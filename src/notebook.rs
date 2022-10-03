use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use ShellHandler;
use crate::shell::shell_handler::FtDictEntry;
use crate::shell::nanoshell::Nanoshell;

use crate::notebook_json::notebook_save;
use crate::notebook_json::notebook_load;

// NotebookEntry
#[derive(Serialize, Deserialize)]
pub struct NotebookEntry {
    pub name: String,
    pub description: String,
}

// Notebook
pub struct Notebook<'a> {
    file: &'a str,
    notes: HashMap<String, NotebookEntry>,
    shell: Nanoshell<'a>,
}

// Constructor
impl<'a> Notebook<'a> {
    fn cmd_dict() -> Vec<FtDictEntry> {
        vec![
            FtDictEntry {
                name: String::from("list"),
                man: String::from("Show all the elements."),
            },
            FtDictEntry {
                name: String::from("add"),
                man: String::from("Opens a form to fill all the data."),
            },
            FtDictEntry {
                name: String::from("remove"),
                man: String::from("Removes a element by name."),
            },
        ]
    }

    pub fn new(file: &'a str) -> Self {
        let mut n = Notebook {
            file: file,
            notes: HashMap::new(),
            shell: Nanoshell {
                title: "Rust-Notebook\n\n",
                promt: "\x1b[38;5;33m$>\x1b[0m ",
                cmd_handler: ShellHandler {
                    cmd_dict: Notebook::cmd_dict(),
                },
            },
        };
        notebook_load(n.file, &mut n.notes);
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
                "list" => self.list(),
                "add" => self.add(),
                "remove" => self.remove(),
                &_ => todo!(),
            }
        }
        self.save_session();
    }

    // Commands

    fn list(&self) {
        self.shell.print_buffered("List:\n--------------------------\n");
        for (_, entry) in &self.notes {
            self.shell.print_buffered("- ");
            self.shell.print_buffered(&entry.name);
            self.shell.print_buffered("\n  ");
            self.shell.print_buffered(&entry.description);
            self.shell.print_buffered("\n");
        }
        self.shell.print("--------------------------\n");
    }

    fn add(&mut self) {
        let name: String = self.shell.ask("Name:\n  ");
        let description: String = self.shell.ask("Description:\n  ");

        self.notes.insert(
            String::from(&name),
            NotebookEntry {
                name: name,
                description: description,
            },
        );
    }

    fn remove(&mut self) {
        let name: String = self.shell.ask("Name:\n  ");

        if !self.notes.contains_key(&name) {
            self.shell.print_buffered("Note '");
            self.shell.print_buffered(&name);
            self.shell.print("' does not exists.\n");
        }
        else {
            self.notes.remove(&name);
            self.shell.print_buffered(&name);
            self.shell.print(" removed.\n");
        }
    }

    // Session
    fn save_session(&self) {
        notebook_save(self.file, &self.notes);
    }
}
