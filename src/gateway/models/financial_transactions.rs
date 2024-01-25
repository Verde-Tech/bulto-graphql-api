use async_graphql::*;

use super::*;

#[derive(SimpleObject)]
pub struct Transaction {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    // Additional transaction fields
}

#[derive(InputObject)]
pub struct TransactionInput {
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: TransactionType,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct TransactionResult {
    pub transaction: Transaction,
    // Additional result fields
}
