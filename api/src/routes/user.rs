use std::sync::{Arc, Mutex};
use poem::{
    handler, http::StatusCode, web::{Data, Json}, Error
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::req_input::CreateUserInput;
use crate::req_output::{CreateUserResponse, SignInResponse};
use store::store::Store;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}



#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<CreateUserResponse>, Error> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).map_err(|_| Error::from_status(StatusCode::CONFLICT))?;
    let response = CreateUserResponse {
        id,
    };

    Ok(Json(response))
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<SignInResponse>, Error> {
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username, data.password);
    
    match user_id {
        Ok(id) => {
            let token = encode(&Header::default(), &Claims {
                sub: id.to_string(),
                exp: 1000000000,
            }, &EncodingKey::from_secret(b"xyz")).map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

            Ok(Json(SignInResponse {
                jwt: token,
            }))
        }
        Err(_e) => {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
    }
}