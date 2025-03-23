use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = Database::connect(database_url).await.expect("Database connection failed");

    println!("✅ Successfully connected to the database!");

    database
}
