use async_graphql::*;

// GraphQL types for User Management Service
#[derive(SimpleObject)]
pub struct UserAccount {
    user_id: ID,
    username: String,
    email: String,
    // Add other user account fields
}

// GraphQL types for Financial Transactions Service
#[derive(SimpleObject)]
pub struct Transaction {
    transaction_id: ID,
    user_id: ID,
    amount: f64,
    transaction_type: String,
    // Add other transaction fields
}

// GraphQL types for Credit Scoring Service
#[derive(SimpleObject)]
pub struct CreditScore {
    user_id: ID,
    score: i32,
    // Add other credit score fields
}

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
