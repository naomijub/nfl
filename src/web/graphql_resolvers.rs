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
enum Sorting {
    TotalRushingYards,
    LongestRush,
    TotalRushingTouchdowns,
}

#[derive(juniper::GraphQLEnum)]
enum Order {
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
        let players = context
            .0
            .iter()
            .skip((per_page * page) as usize)
            .take(per_page as usize)
            .map(|p| p.to_owned());
        Ok(players.collect::<Vec<Player>>())
    }

    fn playersByName(
        context: &Context,
        per_page: i32,
        page: i32,
        pattern: String,
    ) -> Result<Vec<Player>, Error> {
        let players = context
            .0
            .iter()
            .filter(|p| p.name.starts_with(&pattern))
            .skip((per_page * page) as usize)
            .take(per_page as usize)
            .map(|p| p.to_owned());
        Ok(players.collect::<Vec<Player>>())
    }

    fn sortPlayers(
        context: &Context,
        per_page: i32,
        page: i32,
        sort_by: Sorting,
        order: Order,
    ) -> Result<Vec<Player>, Error> {
        let mut players = context
            .0
            .iter()
            .skip((per_page * page) as usize)
            .take(per_page as usize)
            .map(|p| p.to_owned())
            .collect::<Vec<Player>>();
        match (order, sort_by) {
            (Order::Asc, Sorting::TotalRushingYards) => players.sort_by(|a, b| {
                a.total_rushing_yards
                    .partial_cmp(&b.total_rushing_yards)
                    .unwrap()
            }),
            (Order::Asc, Sorting::LongestRush) => {
                players.sort_by(|a, b| a.longest_rush().partial_cmp(&b.longest_rush()).unwrap())
            }
            (Order::Asc, Sorting::TotalRushingTouchdowns) => players.sort_by(|a, b| {
                a.total_rushing_touchdowns
                    .partial_cmp(&b.total_rushing_touchdowns)
                    .unwrap()
            }),
            (Order::Desc, Sorting::TotalRushingYards) => players.sort_by(|b, a| {
                a.total_rushing_yards
                    .partial_cmp(&b.total_rushing_yards)
                    .unwrap()
            }),
            (Order::Desc, Sorting::LongestRush) => {
                players.sort_by(|b, a| a.longest_rush().partial_cmp(&b.longest_rush()).unwrap())
            }
            (Order::Desc, Sorting::TotalRushingTouchdowns) => players.sort_by(|b, a| {
                a.total_rushing_touchdowns
                    .partial_cmp(&b.total_rushing_touchdowns)
                    .unwrap()
            }),
        }
        Ok(players)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}
