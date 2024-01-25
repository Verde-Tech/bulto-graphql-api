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
#[derive(SimpleObject)]
pub struct CreditReport {
    pub user_id: ID,
    pub credit_score: i32,
    pub credit_limit: f64,
    // Additional fields for credit reports
}

#[derive(SimpleObject)]
pub struct CreditScoreFactors {
    pub payment_history: f64,
    pub debt_burden: f64,
    pub length_of_credit_history: f64,
    pub types_of_credit_used: f64,
    pub recent_credit_inquiries: f64,
    // Additional fields for credit score factors
}

#[derive(InputObject)]
pub struct CreditScoreInput {
    pub user_id: ID,
    // Additional input fields for credit score calculation
}
