use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_pw: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hashed_pw: &'a str,
}
