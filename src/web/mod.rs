pub mod graphql_resolvers;
pub mod routes;

use crate::model::json::Player;
use graphql_resolvers::Resolver;

pub struct Container {
    pub context: Vec<Player>,
    pub resolvers: Resolver,
}

impl Container {
    pub fn new(context: Vec<Player>, resolvers: Resolver) -> Self {
        Container {context, resolvers}
    }
}