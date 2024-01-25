use async_graphql::*;

use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use models::*;

mod models;
mod error;

pub type MySchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

}

pub fn schema() -> MySchema {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
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
