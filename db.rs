use std::env;
use rusqlite::{Connection, Result};
use postgres::{Client, NoTls};

fn connect_to_postgresql() -> Result<Client, postgres::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client = Client::connect(&database_url, NoTls)?;
    Ok(client)
}

fn connect_to_sqlite() -> Result<Connection, rusqlite::Error> {
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
    let conn = Connection::open(database_path)?;
    Ok(conn)
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