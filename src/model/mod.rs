use super::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct FreshUser {
    username: String,
}

impl FreshUser {
    pub fn new(username: &str) -> Self {
        Self { username: username.to_string() }
    }
}
