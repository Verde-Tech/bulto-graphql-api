use async_graphql::*;

use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod models;
mod error;

//TODO Replace EmptyMutation with your actual Mutation type.
//TODO Replace EmptySubscription with your actual Subscription type.
pub type MySchema = Schema<Query, Mutation, Subscription>;

pub struct Mutation;

#[Object]
impl Mutation {
    // Define mutation methods here
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    // Define subscription methods here
}

pub struct Query;

#[Object]
impl Query {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

    // User Management Service
    async fn user_account(&self, user_id: ID) -> Result<UserAccount> {
        // Fetch user account logic here
    }

    // Financial Transactions Service
    async fn transaction_history(&self, user_id: ID) -> Result<Vec<Transaction>> {
        // Fetch transaction history logic here
    }

    // Credit Scoring Service
    async fn credit_score(&self, user_id: ID) -> Result<CreditScore> {
        // Calculate credit score logic here
    }

    // Add more methods here for each field in the Query type in the GraphQL schema.
    // Each method should return a Result and use the models to fetch the data.
}

// GraphQL types for User Management Service
#[derive(SimpleObject)]
pub struct UserAccount {
    user_id: ID,
    username: String,
    email: String,
    // Add other user account fields
}

// GraphQL types for Financial Transactions Service
#[derive(SimpleObject)]
pub struct Transaction {
    transaction_id: ID,
    user_id: ID,
    amount: f64,
    transaction_type: String,
    // Add other transaction fields
}

// GraphQL types for Credit Scoring Service
#[derive(SimpleObject)]
pub struct CreditScore {
    user_id: ID,
    score: i32,
    // Add other credit score fields
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
