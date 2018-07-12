use chrono::NaiveDateTime;
use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::Roles;
use schema::users;
use utils::errors::{DatabaseError, ErrorCode};
use utils::passwords::PasswordHash;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pw: String,
    pub created_at: NaiveDateTime,
    pub last_used: Option<NaiveDateTime>,
    pub active: bool,
    pub role: Vec<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pw: String,
    role: Vec<String>,
}

impl NewUser {
    pub fn commit(&self, conn: &Connectable) -> Result<User, DatabaseError> {
        let res = diesel::insert_into(users::table)
            .values(self)
            .get_result(conn.get_connection());
        DatabaseError::wrap(ErrorCode::InsertError, "Could not create new user", res)
    }
}

impl User {
    pub fn create(name: &str, email: &str, phone: &str, password: &str) -> NewUser {
        let hash = PasswordHash::generate(password, None);
        NewUser {
            name: String::from(name),
            email: String::from(email),
            phone: String::from(phone),
            hashed_pw: hash.to_string(),
            role: vec![Roles::Guest.to_string()],
        }
    }

    pub fn check_password(&self, password: &str) -> bool {
        let hash = match PasswordHash::from_str(&self.hashed_pw) {
            Ok(h) => h,
            Err(_) => return false,
        };
        hash.verify(password)
    }
}
