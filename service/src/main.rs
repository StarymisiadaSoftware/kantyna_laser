use actix_web::{post,App, HttpResponse, HttpServer, Responder, ResponseError, dev::Service};
use serde::Deserialize;
use thiserror::Error;
use std::{path::Path, borrow::Borrow};
use anyhow::Result;
use serde_json::from_str;
use tokio::fs::OpenOptions;

async fn append_to_file(a: &Path, data: &[u8]) {

}

#[derive(Debug,Deserialize)]
struct EnqueueRequest {
    url: String
}


#[derive(Debug,Error)]
enum MyError {
    #[error("Failed to deserialize the JSON: {0}")]
    DeserializationError(#[from] serde_json::Error)
}

impl ResponseError for MyError {
    //nothing
}


#[post("/enqueue")]
async fn enqueue(req_body: String) -> Result<String,MyError> {
    eprintln!("Received: {}",&req_body);
    let enqueue_request : EnqueueRequest = from_str(&req_body)?;
    Ok(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(enqueue)
            .wrap_fn(|req,srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    eprintln!("{}: {:?}",res.response().status(),res.response().body());
                    Ok(res)
            }})
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}