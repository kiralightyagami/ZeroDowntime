use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignInResponse {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetWebsiteResponse {
    pub url: String,
}