use crate::logic::{list_all_players, players_by_name, sort_by_name, sort_players};
use crate::model::{error::Error, json::Player};
use juniper::FieldResult;
use juniper::RootNode;
use std::sync::Arc;

pub struct Context(Arc<Vec<Player>>);

impl Context {
    pub fn new(players: Arc<Vec<Player>>) -> Self {
        Context { 0: players }
    }
}
impl juniper::Context for Context {}

#[derive(juniper::GraphQLEnum)]
pub enum Sorting {
    TotalRushingYards,
    LongestRush,
    TotalRushingTouchdowns,
}

#[derive(juniper::GraphQLEnum)]
pub enum Order {
    Asc,
    Desc,
}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    fn listPlayers(context: &Context, per_page: i32, page: i32) -> Result<Vec<Player>, Error> {
        let players = context.0.to_vec();
        Ok(list_all_players(players, per_page, page))
    }

    fn playersByName(
        context: &Context,
        per_page: i32,
        page: i32,
        pattern: String,
    ) -> Result<Vec<Player>, Error> {
        let players = context.0.to_vec();
        Ok(players_by_name(players, per_page, page, pattern))
    }

    fn sortPlayers(
        context: &Context,
        per_page: i32,
        page: i32,
        sort_by: Sorting,
        order: Order,
    ) -> Result<Vec<Player>, Error> {
        let players = context.0.to_vec();
        sort_players(players, per_page, page, sort_by, order)
    }

    fn sortByName(
        context: &Context,
        per_page: i32,
        page: i32,
        pattern: String,
        sort_by: Sorting,
        order: Order,
    ) -> Result<Vec<Player>, Error> {
        let players = context.0.to_vec();
        sort_by_name(players, per_page, page, pattern, sort_by, order)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}
