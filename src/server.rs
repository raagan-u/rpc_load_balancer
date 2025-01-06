use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use std::env;

async fn auth_handler(port: web::Data<u16>) -> impl Responder {
    HttpResponse::Ok().json(format!("up and running on port {:#?}", port))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse port");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(port))
            .route("/", web::get().to(auth_handler))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
