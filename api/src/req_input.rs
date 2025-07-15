use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsite {
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
}

