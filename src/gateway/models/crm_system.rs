use async_graphql::*;

/// Represents a customer in the CRM system.
#[derive(SimpleObject)]
pub struct Customer {
    /// Unique identifier for the customer.
    pub id: ID,
    /// Full name of the customer.
    pub name: String,
    /// Email of the customer
    pub email: String,
    /// Phone number of the customer.
    pub phone: String,
    // Additional customer fields
}

/// Input for creating or updating customer information.
#[derive(InputObject)]
pub struct CustomerInput {
    /// Full name of the customer.
    pub name: String,
    /// Email of the customer
    pub email: String,
    /// Phone number of the customer.
    pub phone: String,
    // Additional input fields for customer
}

#[derive(SimpleObject)]
pub struct MarketingTool {
    /// Unique identifier for the marketing tool.
    pub id: ID,
    /// Full name of the customer.
    pub name: String,
    /// Description of the marketing tool.
    pub description: String,
    // Additional marketing tool fields
}

