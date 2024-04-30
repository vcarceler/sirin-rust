//
// rust-sirin <IP> <PORT> <SECRET>

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}


#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("sirin-rust")
}

#[post("/launcher")]
async fn launcher(req_body: String) -> impl Responder {

    HttpResponse::Ok().body(req_body)
}

#[get("/collect")]
async fn collect(req: HttpRequest) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        println!("Client address {:?}", val.ip());
        println!("Client port {:?}", val.port());
    };
    HttpResponse::Ok()
}

#[get("/register/{name}")]
async fn register(name: web::Path<String>, req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    println!("sirin-rust /register/{}", name);
    println!("AppState.app_name: {}", &data.app_name);
    if let Some(val) = req.peer_addr() {
        println!("Client address {:?}", val.ip());
        println!("Client port {:?}", val.port());
    };
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ip = std::env::args().nth(1).expect("No se ha indicado IP");
    let port = std::env::args().nth(2).expect("No se ha indicado puerto");
    let secreto = std::env::args().nth(3).expect("No se ha indicado secreto");

    println!("sirin-rust IP PORT SECRET");
    println!("sirin-rust {0} {1} {2}", ip, port, secreto);

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(launcher)
            .service(collect)
            .service(register)
    })
    .workers(1) // Solo queremos 1 hilo
    .bind((ip, port.parse::<u16>().unwrap()))?
    .run()
    .await

}
