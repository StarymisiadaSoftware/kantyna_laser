use actix_web::{post, App, HttpResponse, HttpServer, Responder};


#[post("/enqueue")]
async fn enqueue(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(enqueue)
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}