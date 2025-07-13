use poem::{
    get, handler, listener::TcpListener, post, web::{Json,  Path},  Route, Server
};

use crate::req_input::CreateWebsite;
use crate::req_output::CreateWebsiteResponse;

pub mod req_input;
pub mod req_output;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name) 
}

#[handler]
fn create_website(Json(data): Json<CreateWebsite>) -> Json<CreateWebsiteResponse> {
    let url = data.url;

    let response = CreateWebsiteResponse {
        id: "123".to_string(),
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