use async_graphql::*;

// GraphQL types for Lending Service
#[derive(SimpleObject)]
pub struct LoanApplication {
    application_id: ID,
    user_id: ID,
    amount: f64,
    status: String,
    // Add other loan application fields
}

// GraphQL types for Card Integration Service
#[derive(SimpleObject)]
pub struct CardDetails {
    card_id: ID,
    user_id: ID,
    card_number: String,
    card_type: String,
    expiration_date: String,
    // Add other card details fields
}

// GraphQL types for Point of Sale Service
#[derive(SimpleObject)]
pub struct SaleTransaction {
    transaction_id: ID,
    user_id: ID,
    amount: f64,
    merchant_id: ID,
    timestamp: String,
    // Add other sale transaction fields
}

// Add more models here for each service
