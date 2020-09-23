use actix_web::{test, App};
use bytes::Bytes;
use nfl_rushing::web::routes::routes;

#[actix_rt::test]
async fn test_ping_pong() {
    let container = test_data();
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

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
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

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
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_per_page_3_page_0())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_per_page_3_page_0()));
}

#[actix_rt::test]
async fn test_player_by_name() {
    let container = test_data();
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_player_by_name())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_player_by_name()));
}

#[actix_rt::test]
async fn test_players_sorted_ld() {
    let container = test_data();
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_players_sorted_ld())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_players_sorted_ld()));
}

#[actix_rt::test]
async fn test_players_sorted_yds() {
    let container = test_data();
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_players_sorted_yds())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_players_sorted_yds()));
}

#[actix_rt::test]
async fn test_players_sorted_td() {
    let container = test_data();
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_players_sorted_td())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_players_sorted_td()));
}

#[actix_rt::test]
async fn test_sort_player_by_name() {
    let container = test_data();
    let mut app = test::init_service(App::new().data(container.clone()).configure(routes)).await;

    let req = test::TestRequest::post()
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .set_payload(query_sort_td_by_name())
        .to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(response_sort_players_name()));
}

use nfl_rushing::reader::read_json;
use nfl_rushing::web::{graphql_resolvers::create_resolver, Container};
use std::sync::Arc;

pub fn test_data() -> Arc<Container> {
    let resolvers = create_resolver();
    let json = read_json("rushing.json").expect("Failed to init project");

    std::sync::Arc::new(Container::new(json, resolvers))
}

fn query_per_page_2_page_1() -> &'static str {
    "{\"query\": \"query { listPlayers(perPage: 2, page: 1) { name, team, position, longestRush, }}\"}"
}

fn response_per_page_2_page_1() -> &'static [u8; 178] {
    b"{\"data\":{\"listPlayers\":[{\"name\":\"Breshad Perriman\",\"team\":\"BAL\",\"position\":\"WR\",\"longestRush\":\"2\"},{\"name\":\"Charlie Whitehurst\",\"team\":\"CLE\",\"position\":\"QB\",\"longestRush\":\"2\"}]}}"
}

fn query_per_page_3_page_0() -> &'static str {
    "{\"query\": \"query { listPlayers(perPage: 2, page: 1) { name, position, attemps}}\"}"
}

fn response_per_page_3_page_0() -> &'static [u8; 140] {
    b"{\"data\":{\"listPlayers\":[{\"name\":\"Breshad Perriman\",\"position\":\"WR\",\"attemps\":1},{\"name\":\"Charlie Whitehurst\",\"position\":\"QB\",\"attemps\":2}]}}"
}

fn query_player_by_name() -> &'static str {
    "{\"query\": \"query { playersByName(perPage: 3, page: 0, pattern: \\\"A\\\") { name }}\"}"
}

fn response_player_by_name() -> &'static [u8; 107] {
    b"{\"data\":{\"playersByName\":[{\"name\":\"Aaron Ripkowski\"},{\"name\":\"Antonio Brown\"},{\"name\":\"Adrian Peterson\"}]}}"
}

fn query_players_sorted_ld() -> &'static str {
    "{\"query\": \"query { sortPlayers(perPage: 3, page: 0, sortBy: LONGEST_RUSH, order: DESC) { name, longestRush}}\"}"
}

fn response_players_sorted_ld() -> &'static [u8; 153] {
    b"{\"data\":{\"sortPlayers\":[{\"name\":\"Shaun Hill\",\"longestRush\":\"9\"},{\"name\":\"Joe Banyard\",\"longestRush\":\"7\"},{\"name\":\"Breshad Perriman\",\"longestRush\":\"2\"}]}}"
}

fn query_players_sorted_yds() -> &'static str {
    "{\"query\": \"query { sortPlayers(perPage: 3, page: 0, sortBy: TOTAL_RUSHING_YARDS, order: ASC) { name, totalRushingYards}}\"}"
}

fn response_players_sorted_yds() -> &'static [u8; 165] {
    b"{\"data\":{\"sortPlayers\":[{\"name\":\"Breshad Perriman\",\"totalRushingYards\":2},{\"name\":\"Shaun Hill\",\"totalRushingYards\":5},{\"name\":\"Joe Banyard\",\"totalRushingYards\":7}]}}"
}

fn query_players_sorted_td() -> &'static str {
    "{\"query\": \"query {sortPlayers(perPage: 3, page: 10, sortBy: TOTAL_RUSHING_TOUCHDOWNS, order: DESC) { name, totalRushingTouchdowns}}\"}"
}

fn response_players_sorted_td() -> &'static [u8; 182] {
    b"{\"data\":{\"sortPlayers\":[{\"name\":\"Tyrod Taylor\",\"totalRushingTouchdowns\":6},{\"name\":\"Antonio Brown\",\"totalRushingTouchdowns\":0},{\"name\":\"Javorius Allen\",\"totalRushingTouchdowns\":0}]}}"
}

fn query_sort_td_by_name() -> &'static str {
    "{\"query\": \"query {sortByName(perPage: 3, page: 0,  pattern: \\\"A\\\", sortBy: TOTAL_RUSHING_TOUCHDOWNS, order: DESC) { name, totalRushingTouchdowns}}\"}"
}

fn response_sort_players_name() -> &'static [u8; 185] {
    b"{\"data\":{\"sortByName\":[{\"name\":\"Aaron Ripkowski\",\"totalRushingTouchdowns\":2},{\"name\":\"Antonio Brown\",\"totalRushingTouchdowns\":0},{\"name\":\"Adrian Peterson\",\"totalRushingTouchdowns\":0}]}}"
}
