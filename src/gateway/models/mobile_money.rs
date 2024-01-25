use async_graphql::*;

#[derive(SimpleObject)]
pub struct MobileTransaction {
    pub id: ID,
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    // Additional mobile transaction fields
}

#[derive(InputObject)]
pub struct MobileTransactionInput {
    pub user_id: ID,
    pub amount: f64,
    pub transaction_type: TransactionType,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct MobileTransactionResult {
    pub mobile_transaction: MobileTransaction,
    // Additional result fields
}
use async_graphql::*;

#[derive(SimpleObject)]
pub struct InternationalMoneyTransfer {
    pub id: ID,
    pub sender_user_id: ID,
    pub receiver_user_id: ID,
    pub amount: f64,
    pub currency: String,
    pub status: String, // "pending", "completed", "failed"
    // Additional fields for international money transfers
}

#[derive(InputObject)]
pub struct InternationalMoneyTransferInput {
    pub sender_user_id: ID,
    pub receiver_user_id: ID,
    pub amount: f64,
    pub currency: String,
    // Additional input fields
}

#[derive(SimpleObject)]
pub struct ContractorPayment {
    pub id: ID,
    pub contractor_user_id: ID,
    pub amount: f64,
    pub currency: String,
    pub status: String, // "pending", "completed", "failed"
    // Additional fields for contractor payments
}

#[derive(InputObject)]
pub struct ContractorPaymentInput {
    pub contractor_user_id: ID,
    pub amount: f64,
    pub currency: String,
    // Additional input fields
}

