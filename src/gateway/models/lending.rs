use async_graphql::*;
use super::*;

#[derive(SimpleObject)]
pub struct Loan {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub status: LoanStatus,
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
#[derive(SimpleObject)]
pub struct LoanApplication {
    pub user_id: ID,
    pub amount: f64,
    pub term: i32,
    pub interest_rate: f64,
    // Additional fields for loan applications
}

#[derive(SimpleObject)]
pub struct LoanApproval {
    pub loan_id: ID,
    pub approved_amount: f64,
    pub approved_term: i32,
    pub approved_interest_rate: f64,
    // Additional fields for loan approvals
}

#[derive(InputObject)]
pub struct LoanApprovalInput {
    pub loan_id: ID,
    pub approved: bool,
    // Additional input fields for loan approvals
}

