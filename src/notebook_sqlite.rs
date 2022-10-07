use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

use std::fs;
use std::io;
use std::io::Write;

pub struct NotebookDB {
    pub file: String,
    file_url: String,
}

impl NotebookDB {
    pub async fn new(file: &str) -> Self {
        let db: NotebookDB = NotebookDB {
            file: String::from("sqlite:") + file,
            file_url: String::from(file), 
        };
        db.init_db().await;
        db
    }

    async fn open_db(&self) -> SqlitePool {
        let db = SqlitePool::connect(&self.file).await.unwrap();
        db
    }

    async fn close_db(&self, pool: &SqlitePool) {
        pool.close().await;
    }

    fn is_valid_table(&self, table_type: &String, check_note: bool) -> bool{
        match table_type.as_str() {
            "category" | "tag" => true,
            "note" => check_note,
            _ => false
        }
    }

    // List

    pub async fn list_all(&self, table_type: &String) -> Result<&str, &str> {
        if !self.is_valid_table(table_type, false) {
            return Err("Use category or tag");
        }
        print!("Listing by {table_type}\n");
        let db = self.open_db().await;

        // TODO

        self.close_db(&db).await;
        Err("Not implemented")
    }

    pub async fn list(&self, table_type: &String, t: &String) -> Result<&str, &str>{
        if !self.is_valid_table(table_type, false) {
            return Err("Use category or tag");
        }
        print!("Listing {table_type} {t}\n");
        let db = self.open_db().await;

        // TODO

        self.close_db(&db).await;
        Err("Not implemented")
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
    pub async fn init_db(&self) {
        if !Sqlite::database_exists(&self.file).await.unwrap_or(false) {
            print!("Creating new DB...");
            io::stdout().flush().unwrap();
            Sqlite::create_database(&self.file).await.unwrap();
            // TODO allow the script to be stored somewhere else.
            let script_file = fs::read_to_string("docs/db.sql");
            match script_file {
                Ok(script) => {
                    self.create_db(&script).await;
                },
                Err(e) => {
                    self.init_db_fail(e.to_string());
                },
            }
        }
        else {
            print!("Previous notes recovered.\n");
        }
    }

    async fn create_db(&self, script: &str) {
        let pool = SqlitePool::connect(&self.file).await.unwrap();
        let result = sqlx::query(&script).execute(&pool).await;   
        pool.close().await;
        match result {
            Ok(_) => print!("done!\n"),
            Err(e) => {
                self.init_db_fail(e.to_string());
            },
        }
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
