mod model;
mod reader;
mod web;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web as http_web, App, HttpServer};
use env_logger;

use crate::web::{graphql_resolvers::create_resolver, routes::routes, Container};
use reader::read_json;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let resolvers = create_resolver();
    let json = read_json("rushing.json").expect("Failed to init project");

    let container = std::sync::Arc::new(Container::new(json, resolvers));
    HttpServer::new(move || {
        App::new()
            .data(container.clone())
            .wrap(Logger::new(
                "IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D",
            ))
            .wrap(Cors::default())
            .configure(routes)
            .default_service(http_web::to(|| async { "404" }))
    })
    .workers(num_cpus::get() - 1)
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
