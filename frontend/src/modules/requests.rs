use reqwest::{Response, Client, Method};
use serde::{Serialize, Deserialize};
use serde_json::{};
use std::error::Error;

async fn request<B>(method: Method, path: &str, body: B) -> Response 
where B: Serialize {
    let allow_body = method == Method::POST || method == Method::PUT;
    let mut builder = Client::new()
        .request(method, path)
        .header("Content-Type", "application/json");
    if allow_body {
        builder =   builder.json(&body);
    }
    builder.send().await.unwrap()
}

//pub async fn get_request<B>(path: &str) -> Result<B, Box<dyn Error>>
//where B: Deserialize {

//}