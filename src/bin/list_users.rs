extern crate bigneon_db;
extern crate diesel;
#[macro_use]
extern crate log;
extern crate log4rs;

use self::bigneon_db::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting app");
    info!(target: "db", "Only in log file");
    use bigneon_db::schema::users;
    let connection = bigneon_db::establish_connection();
    let results = users::table
        .filter(users::active.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:10} {}", user.id, user.email);
    }
}
