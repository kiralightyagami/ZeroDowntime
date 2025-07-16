use std::sync::{Arc, Mutex};
use poem::{
        get, handler, listener::TcpListener, post, web::{Data, Json, Path},  Route, Server
};


use crate::req_input::{CreateUserInput, CreateWebsite};
use crate::req_output::{CreateUserResponse, CreateWebsiteResponse, GetWebsiteResponse, SignInResponse};
use store::store::Store;
pub mod req_input;
pub mod req_output;

#[handler]
fn get_website(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteResponse> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    let response = GetWebsiteResponse {
        url: website.url,
    };
    Json(response)
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserResponse> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).unwrap();
    let response = CreateUserResponse {
        id: id,
    };

    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SignInResponse> {
    let mut locked_s = s.lock().unwrap();
    let _id = locked_s.sign_in(data.username, data.password).unwrap();
    let response = SignInResponse {
        jwt: String::from("jwt"),
    };

    Json(response)
}
#[handler]
fn create_website(Json(data): Json<CreateWebsite>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteResponse> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website("550e8400-e29b-41d4-a716-446655440000".to_string(), data.url).unwrap();
    let response = CreateWebsiteResponse {
        id: website.id,
    };

    Json(response)
}


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
   
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/user/sign-up", post(sign_up))
    .at("/user/sign-in", post(sign_in))
    .data(s);


    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("poem-server")
        .run(app)
        .await
}