use reqwest::{Response, Client, Method, Error};
use serde::{Serialize, de::DeserializeOwned};

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

pub async fn post_request<B>(path: &str, body: B) -> Result<B, Error>
where B: DeserializeOwned + Serialize {
    Ok(
        request::<B>(
            Method::POST,
            path,
            body)
            .await
            .json::<B>()
            .await
            .unwrap()
    )
}

pub async fn get_request<B>(path: &str) -> Result<B, Error>
where B: DeserializeOwned {
    Ok(
        request::<()>(
            Method::GET,
            path,
            ())
            .await
            .json::<B>()
            .await?
        )
}
