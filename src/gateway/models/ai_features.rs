use async_graphql::*;

#[derive(SimpleObject)]
pub struct FraudDetectionResult {
    pub transaction_id: ID,
    pub risk_level: String, // "low", "medium", "high"
    // Additional fraud detection fields
}

#[derive(SimpleObject)]
pub struct CustomerServiceInteraction {
    pub interaction_id: ID,
    pub user_id: ID,
    pub resolved: bool,
    // Additional customer service fields
}

#[derive(SimpleObject)]
pub struct BusinessIntelligenceInsight {
    pub insight_id: ID,
    pub description: String,
    // Additional business intelligence fields
}

#[derive(InputObject)]
pub struct FraudDetectionInput {
    pub transaction_id: ID,
    // Additional input fields for fraud detection
}

#[derive(InputObject)]
pub struct CustomerServiceInput {
    pub user_id: ID,
    pub query: String,
    // Additional input fields for customer service
}

#[derive(InputObject)]
pub struct ExternalFinancialToolIntegrationInput {
    pub tool_name: String,
    pub data: String,
    // Additional input fields for external tool integration
}
