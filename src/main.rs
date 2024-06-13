use std::net::TcpListener;
use rustnewsletter::run;

// async fn health_check(_req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok().finish()
// }
//
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener =  TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");;

    run(listener)?
        .await
}