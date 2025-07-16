use std::sync::{Arc, Mutex};
use poem::{
    handler, web::{Data, Json, Path},
};

use crate::req_input::CreateWebsite;
use crate::req_output::{CreateWebsiteResponse, GetWebsiteResponse};
use store::store::Store;

#[handler]
pub fn get_website(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteResponse> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    let response = GetWebsiteResponse {
        url: website.url,
    };
    Json(response)
}


#[handler]
pub fn create_website(Json(data): Json<CreateWebsite>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteResponse> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website("550e8400-e29b-41d4-a716-446655440000".to_string(), data.url).unwrap();
    let response = CreateWebsiteResponse {
        id: website.id,
    };

    Json(response)
}