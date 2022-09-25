use std::collections::HashMap;

use crate::shell::shell_handler::FtDictEntry;
use crate::shell::nanoshell::Nanoshell;

pub fn notebook_cmds() -> Box<[FtDictEntry]> {
    Box::new([
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
    ])
}

// NotebookEntry
pub struct NotebookEntry {
    pub name: String,
    pub description: String,
}

// Notebook
pub struct Notebook<'a> {
    // file, // TODO
    pub notes: HashMap<String, NotebookEntry>,
    pub shell: Nanoshell<'a>,
}

impl Notebook<'_> {
    // TODO use shell print logic

    pub fn run(&self) {
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
        print!("Exiting Notebook\n");
    }

    // Commands

    fn do_nothing(&self) {
        print!("Not implemented\n");
    }

    fn list(&self) {
        self.do_nothing();
    }

    fn add(&self) {
        self.do_nothing();
    }

    fn remove(&self) {
        self.do_nothing();
    }
}

