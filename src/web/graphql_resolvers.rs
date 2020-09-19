use juniper::FieldResult;
use juniper::RootNode;
use crate::model::{
    json::Player,
    error::Error
};

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    fn bestPrices(
        per_page: i32,
        page: i32,
    ) -> Result<Vec<Player>, Error> {
        Ok(Vec::new())
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}