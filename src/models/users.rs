use chrono::NaiveDateTime;
use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::Roles;
use schema::users;
use utils::errors::{DatabaseError, ErrorCode};
use utils::passwords::PasswordHash;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pw: String,
    role: Vec<String>,
}

#[derive(Queryable, Serialize, Identifiable)]
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

#[derive(Deserialize, Serialize)]
pub struct DisplayUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
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

    pub fn find(id: &Uuid, conn: &Connectable) -> Result<User, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading user",
            users::table.find(id).first::<User>(conn.get_connection()),
        )
    }

    pub fn find_by_email(email: &str, conn: &Connectable) -> Result<User, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading user",
            users::table
                .filter(users::email.eq(email))
                .first::<User>(conn.get_connection()),
        )
    }

    pub fn check_password(&self, password: &str) -> bool {
        let hash = match PasswordHash::from_str(&self.hashed_pw) {
            Ok(h) => h,
            Err(_) => return false,
        };
        hash.verify(password)
    }

    pub fn add_role(&self, r: Roles, conn: &Connectable) -> Result<User, DatabaseError> {
        let mut new_roles = self.role.clone();
        new_roles.push(r.to_string());
        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not add role to user",
            diesel::update(self)
                .set(users::role.eq(&new_roles))
                .get_result(conn.get_connection()),
        )
    }

    pub fn for_display(self) -> DisplayUser {
        self.into()
    }
}

impl From<User> for DisplayUser {
    fn from(user: User) -> Self {
        DisplayUser {
            id: user.id,
            name: user.name,
            email: user.email,
            phone: user.phone,
            created_at: user.created_at,
        }
    }
}
