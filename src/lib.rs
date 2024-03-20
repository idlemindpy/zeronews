// use actix_web::dev::Server;
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use std::net::TcpListener;
//
// #[allow(dead_code)]
// #[derive(serde::Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }
//
// async fn health() -> impl Responder {
//     HttpResponse::Ok().finish()
// }
//
// async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
//     HttpResponse::Ok().finish()
// }
//
// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health-check", web::get().to(health))
//             .route("/subscribe", web::post().to(subscribe))
//     })
//     .listen(listener)?
//     .run();
//
//     Ok(server)
// }

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
