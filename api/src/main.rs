use poem::{
    get, handler, listener::TcpListener, post, web::{Json,  Path},  Route, Server
};

use crate::req_input::{CreateUserInput, CreateWebsite};
use crate::req_output::{CreateUserResponse, CreateWebsiteResponse, SignInResponse};
use store::store::Store;
pub mod req_input;
pub mod req_output;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name) 
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserResponse> {
    let mut s = Store::default().unwrap();
    let id = s.sign_up(data.username, data.password).unwrap();
    let response = CreateUserResponse {
        id: id,
    };

    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SignInResponse> {
    let mut s = Store::default().unwrap();
    let id = s.sign_in(data.username, data.password).unwrap();
    let response = SignInResponse {
        jwt: String::from("jwt"),
    };

    Json(response)
}
#[handler]
fn create_website(Json(data): Json<CreateWebsite>) -> Json<CreateWebsiteResponse> {
    let mut s = Store::default().unwrap();
    let website = s.create_website("550e8400-e29b-41d4-a716-446655440000".to_string(), data.url).unwrap();
    let response = CreateWebsiteResponse {
        id: website.id,
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
   
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("poem-server")
        .run(app)
        .await
}