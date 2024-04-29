//
// rust-sirin <IP> <PORT> <SECRET>

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("sirin-rust")
}

#[post("/launcher")]
async fn launcher(req_body: String) -> impl Responder {

    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = std::env::args().nth(1).expect("No se ha indicado IP");
    let port = std::env::args().nth(2).expect("No se ha indicado puerto");
    let secreto = std::env::args().nth(3).expect("No se ha indicado secreto");

    println!("{0} {1} {2}", ip, port, secreto);

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(launcher)
    })
    .workers(1) // Solo queremos 1 hilo
    .bind((ip, port.parse::<u16>().unwrap()))?
    .run()
    .await

}