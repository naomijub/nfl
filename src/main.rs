mod model;
mod reader;
mod web;

// use reader::read_json;
// let json = read_json("rushing.json");

  
use actix_web::{
    // web, 
    App, HttpServer};

use crate::web::{
    routes::routes,
    graphql_resolvers::{create_resolver, Resolver}
};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let resolvers: std::sync::Arc<Resolver> = std::sync::Arc::new(create_resolver());
    HttpServer::new(move || {
        App::new()
            .data(resolvers.clone())
            .configure(routes)
            // .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}