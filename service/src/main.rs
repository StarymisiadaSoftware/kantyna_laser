use actix_cors::Cors;
use actix_web::{dev::Service, get, post, App, HttpResponse, HttpServer, Responder, ResponseError};
use anyhow::Result;
use common::{EnqueueRequest, EnqueueRequestReply, MusicQueuePreview};
use lazy_static::lazy_static;
use serde_json::from_str;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

pub mod util;
use util::*;

pub mod song;
use song::*;
pub mod hook_runner;
use hook_runner::*;

pub mod music_queue;
use music_queue::*;

lazy_static! {
    pub static ref hook_runner_instance: Arc<Mutex<HookRunner>> = Default::default();
    pub static ref music_queue_instance: Arc<Mutex<MusicQueue>> = Default::default();
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
    #[error("Illegal URL: {0}")]
    SanitizationError(String),
    #[error("Forbidden: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("Could not extract info about song: {0}")]
    InfoExtractionError(#[from] YtDlpError)

}

impl ResponseError for MyError {
    //nothing needed
}

#[post("/enqueue")]
async fn enqueue(req_body: String) -> impl Responder {
    eprintln!("Received: {}", &req_body);
    let try_ = async {
        let enqueue_request: EnqueueRequest = from_str(&req_body)?;
        let url = sanitize(enqueue_request.url)?;
        let mut new_song = Song::new(&url);
        new_song.load_from_ytdlp().await?;
        new_song.validate()?;
        let (ttw,pos) = music_queue_instance.lock().await.enqueue(new_song.clone());
        Ok::<(u32,usize,Song), MyError>((ttw,pos,new_song))
    };
    match try_.await {
        Ok((ttw,pos,song)) => {
            let reply = EnqueueRequestReply {
                error_message: None,
                pos_in_queue: Some(pos as u32),
                time_to_wait: Some(ttw),
                song_info: Some(song),
            };
            HttpResponse::Accepted().json(reply)
        }
        Err(e) => {
            let reply = EnqueueRequestReply::from_err(e);
            HttpResponse::Forbidden().json(reply)
        }
    }
}

#[get("/preview_queue")]
async fn preview_queue() -> impl Responder {
    let q = music_queue_instance.lock().await.clone();
    HttpResponse::Ok().json(MusicQueuePreview { queue: q })
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    hook_runner_instance
        .lock()
        .await
        .load()
        .await
        .map_err(anyhow_error_to_stdio_error)?;

    tokio::spawn(async {
        loop {
            while let Some(song) = {
                let mut queue = music_queue_instance.lock().await;
                let r = queue.pull_next();
                drop(queue);
                r
            } {
                let hr = hook_runner_instance.lock().await;
                hr.run_hooks(&song.url).await;
            }

            // todo: make it smarter
            // Use a condvar or something
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    HttpServer::new(|| {
        App::new()
            .service(enqueue)
            .service(preview_queue)
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
