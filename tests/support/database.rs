use bigneon_db::db;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

const TEST_DATABASE_URL: &str = "TEST_DATABASE_URL";

pub fn establish_test_connection() -> PgConnection {
    // Load .env, but don't freak out if we can't
    dotenv().ok();

    let database_url =
        env::var(&TEST_DATABASE_URL).expect(&format!("{} must be defined.", TEST_DATABASE_URL));
    let connection = db::establish_connection(&database_url);
    connection.begin_test_transaction().unwrap();
    connection
}
