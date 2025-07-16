use std::sync::{Arc, Mutex};
use poem::{
    handler, web::{Data, Json},
};

use crate::req_input::CreateUserInput;
use crate::req_output::{CreateUserResponse, SignInResponse};
use store::store::Store;


#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserResponse> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).unwrap();
    let response = CreateUserResponse {
        id: id,
    };

    Json(response)
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SignInResponse> {
    let mut locked_s = s.lock().unwrap();
    let _id = locked_s.sign_in(data.username, data.password).unwrap();
    let response = SignInResponse {
        jwt: String::from("jwt"),
    };

    Json(response)
}