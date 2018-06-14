#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

const DB_URL: &str = "DATABASE_URL";

pub fn establish_connection() -> PgConnection {
    // Load .env, but don't freak out if we can't
    dotenv().ok();
    let db_url = env::var(&DB_URL)
        .expect(&format!("{} must be defined.", DB_URL));
    PgConnection::establish(&db_url)
        .expect(&format!("Connection to {} could not be established.", db_url))
}

