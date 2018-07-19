use chrono::{NaiveDateTime, Utc};
use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::Roles;
use schema::users;
use utils::errors::{DatabaseError, ErrorCode};
use utils::passwords::PasswordHash;
use uuid::Uuid;

const PASSWORD_RESET_EXPIRATION_PERIOD_IN_DAYS: i64 = 2;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pw: String,
    role: Vec<String>,
}

#[derive(Queryable, Identifiable)]
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
    pub password_reset_token: Option<Uuid>,
    pub password_reset_requested_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "users"]
pub struct Resetable {
    pub password_reset_token: Option<Uuid>,
    pub password_reset_requested_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
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

    pub fn find_by_password_reset_token(password_reset_token: &Uuid, conn: &Connectable) -> Result<User, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading user",
            users::table
                .filter(users::password_reset_token.eq(password_reset_token))
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

    pub fn create_password_reset_token(&self, conn: &Connectable) -> Result<User, DatabaseError> {
        let data = Resetable {
            password_reset_token: Some(Uuid::new_v4()),
            password_reset_requested_at: Some(Utc::now().naive_utc()),
        };

        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not create token for resetting password",
            diesel::update(self)
                .set(data)
                .get_result(conn.get_connection()),
        )
    }

    pub fn for_display(self) -> DisplayUser {
        self.into()
    }

    pub fn consume_password_reset_token(token: &Uuid, password: &str, conn: &Connectable) -> Result<User, DatabaseError> {
        use schema::users::dsl::*;

        let result = User::find_by_password_reset_token(token, conn);
        match result {
            Ok(user) => {
                let user_id = user.id.clone();
                let token_data : Resetable = user.into();
                if token_data.is_expired() {
                    Err(DatabaseError::new(
                        ErrorCode::InternalError,
                        Some("Password reset token is expired"),
                    ))
                } else {
                    let hash = PasswordHash::generate(password, None);

                    DatabaseError::wrap(
                        ErrorCode::UpdateError,
                        "Could not save new password for user",
                        diesel::update(users.filter(id.eq(user_id)))
                            .set((
                                hashed_pw.eq(&hash.to_string()),
                                Resetable {
                                    password_reset_token: None,
                                    password_reset_requested_at: None,
                                }
                            ))
                            .get_result(conn.get_connection()),
                    )
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Resetable {
    pub fn is_expired(&self) -> bool {
        match self.password_reset_requested_at {
            Some(password_reset_requested_at) => {
                let now = Utc::now().naive_utc();
                now.signed_duration_since(password_reset_requested_at).num_days() >= PASSWORD_RESET_EXPIRATION_PERIOD_IN_DAYS
            }
            None => panic!("Date of password reset not available"),
        }
    }
}

impl From<User> for Resetable {
    fn from(user: User) -> Self {
        Resetable {
            password_reset_token: user.password_reset_token,
            password_reset_requested_at: user.password_reset_requested_at,
        }
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
