use async_graphql::*;

#[derive(SimpleObject)]
pub struct Loan {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub status: String, // "applied", "approved", "disbursed", "repaid"
    // Additional loan fields
}

#[derive(InputObject)]
pub struct LoanApplicationInput {
    pub user_id: ID,
    pub amount: f64,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct LoanResult {
    pub loan: Loan,
    // Additional result fields
}
