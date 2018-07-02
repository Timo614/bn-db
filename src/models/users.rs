use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use schema::users;
use utils::errors::DatabaseError;
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
    pub role: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pw: String,
}

impl NewUser {
    pub fn create(&self, connection: &PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new user")
    }
}

impl User {
    pub fn new(
        name: &str,
        email: &str,
        phone: &str,
        password: &str,
    ) -> Result<NewUser, DatabaseError> {
        let hash = PasswordHash::generate(password, None);
        Ok(NewUser {
            name: String::from(name),
            email: String::from(email),
            phone: String::from(phone),
            hashed_pw: hash.to_string(),
        })
    }

    pub fn check_password(&self, password: &str) -> bool {
        let hash = match PasswordHash::from_str(&self.hashed_pw) {
            Ok(h) => h,
            Err(_) => return false,
        };
        hash.verify(password)
    }
}
