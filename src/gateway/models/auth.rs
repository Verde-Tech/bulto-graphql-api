use async_graphql::*;

#[derive(SimpleObject)]
pub struct LoginResult {
    pub user: User,
    pub token: String,
    pub refresh_token: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
