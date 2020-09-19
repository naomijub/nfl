mod model;
mod reader;
mod web;

use actix_web::{web as http_web, App, HttpServer};

use crate::web::{graphql_resolvers::create_resolver, routes::routes, Container};
use reader::read_json;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let resolvers = create_resolver();
    let json = read_json("rushing.json").expect("Failed to init project");

    let container = std::sync::Arc::new(Container::new(json, resolvers));
    HttpServer::new(move || {
        App::new()
            .data(container.clone())
            .configure(routes)
            .default_service(http_web::to(|| async { "404" }))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
