use async_graphql::*;

#[derive(SimpleObject)]
pub struct MobileTransaction {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: String, // "send_money", "receive_money", "pay_bill"
    pub status: String, // "initiated", "completed", "failed"
    // Additional mobile transaction fields
}

#[derive(InputObject)]
pub struct MobileTransactionInput {
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: String,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct MobileTransactionResult {
    pub mobile_transaction: MobileTransaction,
    // Additional result fields
}
