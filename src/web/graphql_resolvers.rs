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
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}
