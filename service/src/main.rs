use actix_web::{post,App, HttpResponse, HttpServer, Responder, ResponseError, dev::Service};
use common::EnqueueRequest;
use thiserror::Error;
use std::{path::Path, borrow::Borrow};
use anyhow::Result;
use serde_json::from_str;
use actix_cors::Cors;
use tokio::{fs::OpenOptions,io::AsyncWriteExt};

async fn append_to_file(a: &Path, data: &[u8]) -> Result<(),MyError> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(a).await?;
    Ok(file.write_all(data).await?)
}

fn sanitize(youtube_url: String) -> Result<String,MyError> {
    let sanitized = youtube_url;
    Ok(sanitized)
}


#[derive(Debug,Error)]
enum MyError {
    #[error("Failed to deserialize the JSON: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Unable to perform I/O operation: {0}")]
    IoOperationError(#[from] std::io::Error)
}

impl ResponseError for MyError {
    //nothing needed
}


#[post("/enqueue")]
async fn enqueue(req_body: String) -> Result<String,MyError> {
    eprintln!("Received: {}",&req_body);
    let enqueue_request : EnqueueRequest = from_str(&req_body)?;
    let mut url = sanitize(enqueue_request.url)?;
    url.push('\n');
    append_to_file(std::path::Path::new("queue.txt"), url.as_bytes()).await?;
    Ok(url)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(enqueue)
            .wrap(
                Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
            )   
            .wrap_fn(|req,srv| {
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    eprintln!("{}: {:?}",res.response().status(),res.response().body());
                    Ok(res)
            }})
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}