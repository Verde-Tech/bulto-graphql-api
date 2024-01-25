use async_graphql::*;

#[derive(SimpleObject)]
pub struct AIPersonalizedAdvice {
    pub user_id: ID,
    pub advice: String,
    // Additional fields related to personalized advice
}

#[derive(SimpleObject)]
pub struct AIAnalytics {
    pub user_id: ID,
    pub analytics: String,
    // Additional fields related to financial analytics
}
