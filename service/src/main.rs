use actix_cors::Cors;
use actix_web::{dev::Service, post, App, HttpResponse, HttpServer, Responder, ResponseError};
use anyhow::Result;
use common::EnqueueRequest;
use serde_json::from_str;
use std::{path::{Path, PathBuf}};
use thiserror::Error;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

struct Song {
    url: String,
    /// in seconds
    duration: Option<u16>,
    title: Option<String>,
    miniature_url: Option<String>
}

struct MusicQueue {
    queue: Vec<Song>,
    currently_played: Option<Song>
}
struct HookRunner {
    hooks: Vec<PathBuf>
}

impl HookRunner {
    /// Completes when ALL of the hooks have finished processing
    async fn run_hooks(&self) {

    }
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
