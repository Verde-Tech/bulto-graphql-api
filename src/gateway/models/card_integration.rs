use async_graphql::*;

use super::*;

#[derive(SimpleObject)]
pub struct CardTransaction {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    // Additional card transaction fields
}

#[derive(InputObject)]
pub struct CardTransactionInput {
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: TransactionType,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct CardTransactionResult {
    pub card_transaction: CardTransaction,
    // Additional result fields
}
#[derive(SimpleObject)]
pub struct Card {
    pub id: ID,
    pub user_id: ID,
    pub card_number: String,
    pub card_type: String, // "debit", "credit"
    pub expiration_date: String,
    // Additional card fields
}

#[derive(InputObject)]
pub struct CardInput {
    pub user_id: ID,
    pub card_number: String,
    pub card_type: String,
    pub expiration_date: String,
    // Additional input fields for card
}

#[derive(SimpleObject)]
pub struct CardResult {
    pub card: Card,
    // Additional result fields
}
