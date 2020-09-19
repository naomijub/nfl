use nfl_rushing::web::{
    routes::routes,
};
use actix_web::{test, App};
use bytes::Bytes;

#[actix_rt::test]
async fn test_ping_pong() {
    let container = test_data();
    let mut app = test::init_service(
        App::new().data(container.clone()).configure(routes)).await;


    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload("{\"query\": \"query ping { ping }\"}")
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(b"{\"data\":{\"ping\":\"pong\"}}"));
}

#[actix_rt::test]
async fn test_per_page_2_page_1() {
    let container = test_data();
    let mut app = test::init_service(
        App::new().data(container.clone()).configure(routes)).await;


    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_per_page_2_page_1())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_per_page_2_page_1()));
}

#[actix_rt::test]
async fn test_per_page_3_page_0() {
    let container = test_data();
    let mut app = test::init_service(
        App::new().data(container.clone()).configure(routes)).await;


    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_per_page_3_page_0())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_per_page_3_page_0()));
}


use nfl_rushing::web::{
    Container,
    graphql_resolvers::{create_resolver}
};
use nfl_rushing::reader::read_json;
use std::sync::Arc;

pub fn test_data() -> Arc<Container> {
    let resolvers = create_resolver();
    let json = 
        read_json("rushing.json").expect("Failed to init project");

    std::sync::Arc::new(Container::new(json, resolvers))

}

fn query_per_page_2_page_1() -> &'static str {
    "{\"query\": \"query { listPlayers(perPage: 2, page: 1) { name, team, position, largestRush, }}\"}"
}

fn response_per_page_2_page_1() -> &'static [u8; 178] {
    b"{\"data\":{\"listPlayers\":[{\"name\":\"Breshad Perriman\",\"team\":\"BAL\",\"position\":\"WR\",\"largestRush\":\"2\"},{\"name\":\"Charlie Whitehurst\",\"team\":\"CLE\",\"position\":\"QB\",\"largestRush\":\"2\"}]}}"
}

fn query_per_page_3_page_0() -> &'static str {
    "{\"query\": \"query { listPlayers(perPage: 2, page: 1) { name, position, attemps}}\"}"
}

fn response_per_page_3_page_0() -> &'static [u8; 140] {
    b"{\"data\":{\"listPlayers\":[{\"name\":\"Breshad Perriman\",\"position\":\"WR\",\"attemps\":1},{\"name\":\"Charlie Whitehurst\",\"position\":\"QB\",\"attemps\":2}]}}"
}