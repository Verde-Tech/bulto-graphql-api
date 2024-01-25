use async_graphql::*;

#[derive(SimpleObject)]
pub struct Transaction {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: String, // "deposit", "withdrawal", "transfer"
    pub status: String, // "pending", "completed", "failed"
    // Additional transaction fields
}

#[derive(InputObject)]
pub struct TransactionInput {
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: String,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct TransactionResult {
    pub transaction: Transaction,
    // Additional result fields
}
