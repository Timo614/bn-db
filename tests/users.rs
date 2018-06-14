extern crate bigneon_db;
extern crate diesel;

mod common;

use bigneon_db::schema::users;

#[test]
fn add_user() {
    let conn = bigneon_db::establish_connection();
    let email = common::create_random_email();
    let pw = common::create_random_password();
    let user = bigneon_db::create_user(&conn, &email, &pw);
    assert_eq!(user.email, email);
    assert_ne!(user.hashed_pw, pw);
    assert!(user.id > 0);
    assert_eq!(user.active, true);
}