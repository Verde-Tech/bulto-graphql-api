use async_graphql::*;

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    // Additional user fields
}

#[derive(InputObject)]
pub struct NewUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct AuthPayload {
    pub user: User,
    pub token: String,
}
