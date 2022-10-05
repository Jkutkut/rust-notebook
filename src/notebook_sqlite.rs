use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

use std::fs;

pub struct NotebookDB {
    pub file: String,
    file_url: String,
}

impl NotebookDB {
    pub async fn new(file: &str) -> Self {
        let db: NotebookDB = NotebookDB {
            file: String::from("sqlite://") + file,
            file_url: String::from(file), 
        };
        init_db(&db).await;
        db
    }

    pub fn list(&self, category: &String) {
        // TODO
        print!("Listing by {category}");
    }

    pub fn add(&self, category: &String) {
        // TODO
        print!("Adding a {category}\n");
    }

    pub fn remove(&self, category: &String) {
        // TODO
        print!("Removing {category}\n");
    }
}

// Functions:

async fn init_db(db: &NotebookDB) {
    if !Sqlite::database_exists(&db.file).await.unwrap_or(false) {
        print!("Creating new DB...\n");
        Sqlite::create_database(&db.file).await.unwrap();
        let script_file = fs::read_to_string("docs/db.sql"); // TODO allow the script to be stored somewhere else.
        match script_file {
            Ok(script) => {
                let pool = SqlitePool::connect(&db.file).await.unwrap();
                let result = sqlx::query(&script).execute(&pool).await;   
                pool.close().await;
                match result {
                    Ok(_) => print!("Restored!\n  {:?}\n", result),
                    Err(e) => {
                        let mut rm_err: String = String::from("");
                        match fs::remove_file(&db.file_url) { // TODO refactor
                            Ok(_) => (),
                            Err(e) => {
                                rm_err.push_str("\n\nAnother error ocurred:\n");
                                rm_err.push_str(&e.to_string());
                            }
                        }
                        panic!("The script to create the DB failed!\n\n{e}{rm_err}");
                    },
                }
            },
            Err(e) => {
                fs::remove_file(&db.file);
                panic!("Not able to obtain the script to create the DB!\n{e}");
            },
        }
    }
    else {
        print!("Previous notes recovered.\n");
        // TODO check database is correct.
    }
    // let instances = SqlitePool::connect(&db_url).await.unwrap();
    // let qry ="INSERT INTO settings (description) VALUES($1)";
    // let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

    // instances.close().await;

    // println!("{:?}", result);
}
