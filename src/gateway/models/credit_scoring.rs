use async_graphql::*;

#[derive(SimpleObject)]
pub struct CreditScore {
    pub user_id: ID,
    pub score: i32,
    // Additional credit score fields
}

#[derive(SimpleObject)]
pub struct CreditScoreResult {
    pub credit_score: CreditScore,
    // Additional result fields
}
