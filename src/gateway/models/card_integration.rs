use async_graphql::*;

#[derive(SimpleObject)]
pub struct CardTransaction {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: String, // "charge", "refund"
    pub status: String, // "authorized", "captured", "refunded", "failed"
    // Additional card transaction fields
}

#[derive(InputObject)]
pub struct CardTransactionInput {
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: String,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct CardTransactionResult {
    pub card_transaction: CardTransaction,
    // Additional result fields
}
