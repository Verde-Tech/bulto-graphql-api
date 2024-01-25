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
    async fn create_user(&self, ctx: &Context<'_>, input: NewUserInput) -> Result<AuthPayload> {
        // Logic to create a new user and return authentication payload
        todo!()
    }

    async fn update_user(&self, ctx: &Context<'_>, id: ID, input: NewUserInput) -> Result<User> {
        // Logic to update an existing user
        todo!()

    }

    async fn delete_user(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        // Logic to delete a user
        todo!()

    }

    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<AuthPayload> {
        // Logic to authenticate a user and return authentication payload
        todo!()

    }

    async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> Result<AuthPayload> {
        // Logic to refresh the authentication token
        todo!()
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    // Add user management mutations here
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        // Logic to retrieve a user by ID
        todo!()
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        // Logic to retrieve all users
        todo!()
    }
    // Add user management queries here
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
