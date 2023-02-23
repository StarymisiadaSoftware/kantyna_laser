use actix_cors::Cors;
use actix_web::{dev::Service, post, App, HttpResponse, HttpServer, Responder, ResponseError};
use anyhow::Result;
use common::EnqueueRequest;
use lazy_static::lazy_static;
use serde_json::from_str;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command, sync::Mutex};

pub mod util;
use util::*;

pub mod song;
pub mod hook_runner;
use hook_runner::*;

pub mod music_queue;
use music_queue::*;

lazy_static! {
    pub static ref hook_runner_instance: Arc<Mutex<HookRunner>> = Default::default();
    pub static ref music_queue_instance: Arc<Mutex<MusicQueue>> = Default::default();
}


async fn append_to_file(a: &Path, data: &[u8]) -> Result<(), MyError> {
    let mut file = OpenOptions::new().append(true).open(a).await?;
    Ok(file.write_all(data).await?)
}

/// Todo: Only allow youtube urls
fn sanitize(youtube_url: String) -> Result<String, MyError> {
    let sanitized = youtube_url;
    Ok(sanitized)
}

#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to deserialize the JSON: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Unable to perform I/O operation: {0}")]
    IoOperationError(#[from] std::io::Error),
}

impl ResponseError for MyError {
    //nothing needed
}

#[post("/enqueue")]
async fn enqueue(req_body: String) -> Result<String, MyError> {
    eprintln!("Received: {}", &req_body);
    let enqueue_request: EnqueueRequest = from_str(&req_body)?;
    let mut url = sanitize(enqueue_request.url)?;
    url.push('\n');
    append_to_file(std::path::Path::new("queue.txt"), url.as_bytes()).await?;
    Ok(url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    hook_runner_instance
        .lock()
        .await
        .load()
        .await
        .map_err(anyhow_error_to_stdio_error)?;

    HttpServer::new(|| {
        App::new()
            .service(enqueue)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    eprintln!("{}: {:?}", res.response().status(), res.response().body());
                    Ok(res)
                }
            })
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
