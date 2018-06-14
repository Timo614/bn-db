extern crate diesel;
extern crate bigneon_db;

use self::bigneon_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use bigneon_db::schema::users;
    let connection = establish_connection();
    let results = users::table.filter(users::active.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:10} {}", user.id, user.email);
    }
}
