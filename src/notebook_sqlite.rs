// use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};



use std::fs;
use std::io;
use std::io::Write;

pub struct NotebookDB {
    pub file: String,
    file_url: String,
}

impl NotebookDB {
    pub fn new(file: &str) -> Self {
        let db: NotebookDB = NotebookDB {
            file: String::from("sqlite:") + file,
            file_url: String::from(file), 
        };
        db.init_db();
        db
    }

    fn open_db(&self) -> sqlite::Connection {
        let db = sqlite::open(&self.file_url).unwrap();
        db
    }

    fn close_db(&self) {
    }

    fn is_valid_table(&self, table_type: &String, check_note: bool) -> bool{
        match table_type.as_str() {
            "category" | "tag" => true,
            "note" => check_note,
            _ => false
        }
    }

    // List

    pub fn list_all(&self, table_type: &String) -> Result<&str, String> {
        if !self.is_valid_table(table_type, false) {
            return Err(String::from("Use category or tag"));
        }
        print!("Listing by {table_type}\n");
        let db = self.open_db();

        // TODO

        // self.close_db(&db);
        Err(String::from("Not implemented"))
    }

    pub fn list(&self, table_type: &String, t: &String) -> Result<&str, String>{
        if !self.is_valid_table(table_type, false) {
            return Err(String::from("Use category or tag"));
        }
        print!("Listing {table_type} {t}\n");
        let db = self.open_db();

        let qry = "
            SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
            FROM NOTE N, CATEGORY C, TAG T
            WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and C.CAT_NAME == 42;
        "; // TODO
        
        // TODO
        db.iterate(&qry, |note| {
            print!("********\n");
            for &(field, value) in note {
                print!("{}: {:?}\n", field, value);
            }
            print!("********\n");
            true
        }).unwrap();

        Err(String::from("Not implemented"))
    }

    // 
    pub fn add(&self, category: &String) {
        // TODO
        // let instances = SqlitePool::connect(&db_url).await.unwrap();
        // let qry ="INSERT INTO settings (description) VALUES($1)";
        // let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

        // instances.close().await;

        // println!("{:?}", result);
        print!("Adding a {category}\n");
    }

    pub fn remove(&self, category: &String) {
        // TODO
        print!("Removing {category}\n");
    }
}

// NotebookDB tools
impl NotebookDB {
    pub fn init_db(&self) {
        // TODO reimplement
        // if !Sqlite::database_exists(&self.file).await.unwrap_or(false) {
        //     print!("Creating new DB...");
        //     io::stdout().flush().unwrap();
        //     Sqlite::create_database(&self.file).await.unwrap();
        //     // TODO allow the script to be stored somewhere else.
        //     let script_file = fs::read_to_string("docs/db.sql");
        //     match script_file {
        //         Ok(script) => {
        //             self.create_db(&script).await;
        //         },
        //         Err(e) => {
        //             self.init_db_fail(e.to_string());
        //         },
        //     }
        // }
        // else {
        //     print!("Previous notes recovered.\n");
        // }
    }

    fn create_db(&self, script: &str) {
        // TODO reimplement
        // let pool = SqlitePool::connect(&self.file).await.unwrap();
        // let result = sqlx::query(&script).execute(&pool).await;   
        // pool.close().await;
        // match result {
        //     Ok(_) => print!("done!\n"),
        //     Err(e) => {
        //         self.init_db_fail(e.to_string());
        //     },
        // }
    }

    fn init_db_fail(&self, error: String) {
        let mut rm_err: String = String::new();
        match fs::remove_file(&self.file_url) {
            Ok(_) => (),
            Err(e) => {
                rm_err.push_str("\n\nAnother error ocurred:\n");
                rm_err.push_str(&e.to_string());
            }
        }
        panic!("\n\nThe script to create the DB failed!\n\n{error}{rm_err}");
    }
}
