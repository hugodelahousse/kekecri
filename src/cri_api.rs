use crate::models::*;

use maplit::hashmap;
use std::collections::HashMap;
use std::error;

const CRI_URL: &str = "https://cri.epita.fr/api";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub type Authenticator = dyn Fn() -> Result<String>;

pub fn get_jwt(username: &str, password: &str) -> Result<String> {
    let payload: HashMap<&str, &str> = hashmap!(
        "username" => username,
        "password" => password,
    );

    let response: HashMap<String, String> = reqwest::Client::new()
        .post(format!("{}/token-jwt/auth", CRI_URL).as_str())
        .json(&payload)
        .send()?
        .json()?;

    match response.get("token") {
        Some(value) => Ok(value.clone()),
        None => panic!("Invalid response"),
    }
}

fn get_users(limit: u32, offset: u32, authenticator: &Authenticator) -> Result<UsersQueryResult> {
    let result: Option<UsersQueryResult> = reqwest::Client::new()
        .get(format!("{}/users/", CRI_URL).as_str())
        .query(&[("limit", limit), ("offset", offset)])
        .header("Authorization", authenticator()?)
        .send()?
        .json()?;

    Ok(result.unwrap())
}

pub fn get_all_users(batch_size: u32, authenticator: &Authenticator) -> Result<Vec<User>> {
    let mut users: Vec<User> = Vec::new();

    eprintln!("Loading first batch...");
    let mut response = get_users(batch_size, 0, authenticator);
    let total = response?.count;

    // Ceil up division to get last package
    let iterations = 1 + (total - 1) / batch_size;

    for i in 1..iterations {
        eprintln!("Loading users {}/{}...", i * batch_size, total);
        response = get_users(batch_size, batch_size * i, authenticator);
        for user in response?.results {
            users.push(user);
        }
    }

    Ok(users)
}
