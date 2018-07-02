use diesel::prelude::*;

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url).expect(&format!(
        "Connection to {} could not be established.",
        database_url
    ))
}
