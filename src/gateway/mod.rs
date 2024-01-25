use async_graphql::*;

use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod models;
mod error;

//TODO Replace EmptyMutation with your actual Mutation type.
//TODO Replace EmptySubscription with your actual Subscription type.
pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;


pub struct Query;

#[Object]
impl Query {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

    // Add more methods here for each field in the Query type in the GraphQL schema.
    // Each method should return a Result and use the models to fetch the data.
}

pub fn schema() -> MySchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish()
}

pub async fn playground_route() -> HttpResponse {
    let html = async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql_route(schema: web::Data<MySchema>, req: GraphQLRequest) -> web::Json<GraphQLResponse> {
    web::Json(schema.execute(req.into_inner()).await.into())
}
