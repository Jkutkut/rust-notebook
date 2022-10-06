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

    pub fn list(&self, category: &String) {
        // TODO
        print!("Listing by {category}");
    }

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
