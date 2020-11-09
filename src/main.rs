use actix_web::{web, HttpServer, App};
use chat_server::setup_logger::setup_logger;
use actix_web::middleware::Logger;
use chat_server::online_server as server;
use actix::Actor;
use chat_server::my_websocket::ws_index;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    setup_logger().unwrap();

    // Start chat server actor
    let server = server::OnLineServer::default().start();

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(Logger::default())
            .data(server.clone())
            // websocket route
            .service(web::resource("/ws").route(web::get().to(ws_index)))
    })
        // start http server on 127.0.0.1:8080
        .bind("0.0.0.0:8778")?
        .run()
        .await
}
