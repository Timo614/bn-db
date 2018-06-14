#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_pw: String,
    pub active: bool
}