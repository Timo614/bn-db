#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

const DB_URL: &str = "DATABASE_URL";

pub fn establish_connection() -> PgConnection {
    // Load .env, but don't freak out if we can't
    dotenv().ok();
    let db_url = env::var(&DB_URL).expect(&format!("{} must be defined.", DB_URL));
    PgConnection::establish(&db_url).expect(&format!(
        "Connection to {} could not be established.",
        db_url
    ))
}

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, password: &'a str) -> User {
    use schema::users;
    let hashed_pw = hash(&password);
    let new_user = NewUser {
        email: &email,
        hashed_pw: &hashed_pw,
    };
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error creating new user")
}

// replace this with a secure hash + salt or better scheme asap
fn hash(pw: &str) -> String {
    format!("INSECURE {}", pw)
}
