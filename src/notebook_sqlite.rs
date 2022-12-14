use std::fs;
use crate::shell::colors;

use crate::DB_SCRIPT;

pub struct NotebookDB {
    db: sqlite::Connection
}

impl NotebookDB {
    pub fn new(file: &str) -> Self {
        NotebookDB {
            db: NotebookDB::new_db_connection(file)
        }
    }

    // List
    
    fn format_list(&self, mut statement: sqlite::Statement, title: String) -> String {
        let mut empty = true;
        let mut result = title;
        while let Ok(sqlite::State::Row) = statement.next() {
            empty = false;
            result += format!(
                "{}────────────────────────────────────────────────────────────────────╮{}\n\
                {}{}{}\n\n\
                Description:\n\
                {}\n\
                Category: {} Tag: {}\n\
                {}────────────────────────────────────────────────────────────────────╯{}\n",
                colors::BLUE,
                colors::NC,
                colors::YELLOW,
                statement.read::<String>(0).unwrap(), // Name
                colors::NC,
                statement.read::<String>(1).unwrap(), // Description
                statement.read::<String>(2).unwrap(), // Category
                statement.read::<String>(3).unwrap(), // Tag
                colors::BLUE,
                colors::NC,
            ).as_str();
        }
        match empty {
            true => String::from("Nothing found.\n"),
            false => result,
        }
    }

    pub fn list_all(&self, table_type: &str) -> Result<String, String> {
        let query;
        match table_type {
            "category" => query = " 
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID
                ORDER BY C.CAT_NAME",
            "tag" => query = "
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID
                ORDER BY T.TAG_NAME;",
            "all" => query = "
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID
                ORDER BY N.NOTE_NAME;",
            "categories" | "tags" => return self.list_table(table_type),
            _ => return Err(String::from("Use category or tag")),
        }
        let statement = self.db.prepare(query).unwrap();
        Ok(self.format_list(statement, format!("Listing notes by {table_type}\n")))
    }

    pub fn list(&self, table_type: &str, t: &str) -> Result<String, String> {
        let query;
        match table_type {
            "category" => query = " 
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and
                UPPER(C.CAT_NAME) == UPPER(?);",
            "tag" => query = "
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and
                UPPER(T.TAG_NAME) == UPPER(?);",
            _ => return Err(String::from("Use category or tag")),
        }
        let statement = self.db.prepare(query).unwrap().bind(1, t).unwrap();
        Ok(self.format_list(statement, format!("Listing {table_type} {t}\n")))
    }

    pub fn list_table(&self, table_type: &str) -> Result<String, String> {
        let query;
        match table_type {
            "categories" => {
                query = "SELECT ID, CAT_NAME FROM CATEGORY ORDER BY CAT_NAME";
            },
            "tags" => {
                query = "SELECT ID, TAG_NAME FROM TAG ORDER BY TAG_NAME";
            },
            _ => return Err(String::from("Use tags or categories")),
        }
        let mut result = format!("All {table_type}:\n");
        let mut statement = self.db.prepare(query).unwrap();
        while let Ok(sqlite::State::Row) = statement.next() {
            result += format!(
                "{}-{} {} ({})\n",
                colors::BLUE,
                colors::NC,
                statement.read::<String>(1).unwrap(),
                statement.read::<i64>(0).unwrap()
            ).as_str();
        }
        Ok(result)
    }

    // Add

    pub fn add(&self, table_type: &str, t: &str) -> Result<String, String> {
        let query;
        match table_type {
            "category" => query = "INSERT INTO CATEGORY (CAT_NAME) VALUES (?);",
            "tag" => query = "INSERT INTO TAG (TAG_NAME) VALUES (?);",
            _ => return Err(String::from("Use category or tag")),
        }
        let mut statement = self.db.prepare(query).unwrap().bind(1, t).unwrap();
        match statement.next() {
            Ok(_) => Ok(format!("{t} added to the {table_type} list.\n")),
            Err(e) => Err(format!("Not able to add {}:\n{}", t, e.to_string())),
        }
    }

    fn get_id(&self, query: &str, f: &str) -> i64 {
        let mut statement = self.db.prepare(query).unwrap().bind(1, f).unwrap();
        statement.next().unwrap();
        statement.read::<i64>(0).unwrap()
    }

    pub fn add_note(&self, name: &str, description: &str,
                    category: &str, tag: &str) -> Result<String, String> {
        let cat_id = self.get_id("SELECT ID FROM CATEGORY WHERE CAT_NAME = ?;", category);
        if cat_id == 0 {
            return Err(String::from("Category not found."));
        }
        let tag_id = self.get_id("SELECT ID FROM TAG WHERE TAG_NAME = ?;", tag);
        if tag_id == 0 {
            return Err(String::from("Tag not found."));
        }
        let mut s = self.db.prepare("
                INSERT INTO NOTE (NOTE_NAME, NOTE_DESC, CATEGORY_ID, TAG_ID)
                VALUES (?, ?, ?, ?);
            ").unwrap()
            .bind(1, name).unwrap()
            .bind(2, description).unwrap()
            .bind(3, cat_id).unwrap()
            .bind(4, tag_id).unwrap();
        match s.next() {
            Ok(_) => Ok(String::from("Note added.\n")),
            Err(e) => {
                Err(e.to_string())
            },
        }
    }

    pub fn remove(&self, table_type: &str, ele: &str) -> Result<String, String> {
        let query;
        match table_type {
            "category" => query = "
                DELETE FROM CATEGORY WHERE UPPER(CAT_NAME) = UPPER(?);
            ",
            "tag" => query = "
                DELETE FROM TAG WHERE UPPER(TAG_NAME) = UPPER(?);
            ",
            "note" => query = "
                DELETE FROM NOTE WHERE UPPER(NOTE_NAME) = UPPER(?);
            ",
            _ => return Err(String::from("Use category, tag or note.")),
        }
        let mut s = self.db.prepare(query).unwrap().bind(1, ele).unwrap();
        match s.next() {
            Ok(_) => Ok(format!("{} removed.\n", table_type)),
            Err(e) => Ok(format!("There was an error removing the {}:\n  {}", table_type, e.to_string())),
        }
    }
}

// NotebookDB tools
impl NotebookDB {
    pub fn new_db_connection(file: &str) -> sqlite::Connection {
        match std::path::Path::new(file).exists() {
            false => {
                print!("Creating DB...");
                let db;
                match sqlite::open(file) {
                    Ok(c) => db = c,
                    Err(_) => panic!("Not able to open the file."),
                };
                match fs::read_to_string(DB_SCRIPT) {
                    Err(e) => panic!("\nNot able to read the script:\n{}", e.to_string()),
                    Ok(script) => {
                        match db.execute(script) {
                            Ok(_) => {
                                print!(" Done!\n\n");
                                db
                            },
                            Err(e) => panic!("\nNot able to create DB:\n{}", e.to_string()),
                        }
                    },
                }
            },
            true => {
                print!("Restoring previous version...");
                match sqlite::open(file) {
                    Ok(db) => {
                        print!(" Done!\n\n");
                        db
                    }
                    Err(e) => panic!("\nNot able to restore the session:\n{}", e.to_string()),
                }
            }
        }
    }
}
