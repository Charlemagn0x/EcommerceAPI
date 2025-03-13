use std::env;
use rusqlite::{Connection, Result as SqliteResult};
use postgres::{Client, NoTls, Error as PostgresError};

fn connect_to_postgresql() -> Result<Client, PostgresError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&database_url, NoTls).map_err(|e| e)
}

fn connect_to_sqlite() -> SqliteResult<Connection> {
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
    Connection::open(database_path).map_err(|e| e)
}

fn main() {
    dotenv::dotenv().ok();

    match connect_to_postgresql() {
        Ok(_) => println!("Successfully connected to PostgreSQL database"),
        Err(e) => println!("Failed to connect to PostgreSQL database: {}", e),
    }

    match connect_to_sqlite() {
        Ok(_) => println!("Successfully connected to SQLite database"),
        Err(e) => println!("Failed to connect to SQLite database: {}", e),
    }
}