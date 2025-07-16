use std::sync::{Arc, Mutex};
use poem::{
        get, listener::TcpListener, post, Route, Server
};


use crate::routes::website::{get_website, create_website};
use crate::routes::user::{sign_up, sign_in};
use store::store::Store;

pub mod req_input;
pub mod req_output;
pub mod routes;





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