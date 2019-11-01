use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub mail: String,
}

#[derive(Deserialize, Debug)]
pub struct UsersQueryResult {
    pub count: u32,
    pub next: Option<String>,
    pub results: Vec<User>,
}
