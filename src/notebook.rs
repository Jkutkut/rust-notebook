use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::shell::shell_handler::FtDictEntry;
use crate::shell::nanoshell::Nanoshell;

use crate::notebook_json::notebook_save;
use crate::notebook_json::notebook_load;

pub fn notebook_cmds() -> Vec<FtDictEntry> {
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

// NotebookEntry
#[derive(Serialize, Deserialize)]
pub struct NotebookEntry {
    pub name: String,
    pub description: String,
}

// Notebook
pub struct Notebook<'a> {
    // TODO refactor with custom constructor
    pub file: &'a str,
    pub notes: HashMap<String, NotebookEntry>,
    pub shell: Nanoshell<'a>,
}

impl Notebook<'_> {
    pub fn init(&mut self) {
        notebook_load(self.file, &mut self.notes);
    }

    pub fn run(&mut self) {
        self.shell.init();
        let mut cmd: String;
        loop {
            cmd = self.shell.run();
            match cmd.as_str() {
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
        self.shell.print("Name:\n  ");
        let name: String = self.shell.get_input();

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
