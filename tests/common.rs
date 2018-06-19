extern crate bigneon_db;
extern crate diesel;

use diesel::prelude::*;

use bigneon_db::models::{NewUser, User};

pub fn create_random_email() -> String {
    "joe@tari.com".to_string()
}

pub fn create_random_password() -> String {
    "password".to_string()
}
