use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn connect() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

#[derive(Queryable)]
pub struct Ability {
    pub id: usize,
    pub name: String,
    pub base_description: String,
    pub mob: Mob,
}

#[derive(Queryable)]
pub struct Mob {
    pub id: usize,
    pub name: String,
    pub percent: f32,
    pub teeming_percent: f32,
}
