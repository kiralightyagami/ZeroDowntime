use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsite {
    pub url: String,
}


