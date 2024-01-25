use async_graphql::*;

#[derive(SimpleObject)]
pub struct AIPersonalizedAdvice {
    pub user_id: ID,
    pub advice: String,
    // Additional fields related to personalized advice
}

#[derive(SimpleObject)]
pub struct AIAnalytics {
    pub user_id: ID,
    pub analytics: String,
    // Additional fields related to financial analytics
}

#[derive(SimpleObject)]
pub struct FinancialQueryResult {
    pub query: String,
    pub response: String,
    // Additional fields for query results
}

#[derive(InputObject)]
pub struct FinancialQueryInput {
    pub query: String,
    // Additional input fields for financial queries
}

#[derive(SimpleObject)]
pub struct FinancialAdvice {
    pub user_id: ID,
    pub advice: String,
    // Additional fields for financial advice
}

#[derive(SimpleObject)]
pub struct PredictiveAnalyticsResult {
    pub forecast: String,
    // Additional fields for financial forecasting and market trends
}

#[derive(SimpleObject)]
pub struct NaturalLanguageProcessingResult {
    pub query: String,
    pub intent: String,
    pub entities: Vec<String>,
    pub response: String,
    // Additional fields for NLP results
}

#[derive(InputObject)]
pub struct NaturalLanguageProcessingInput {
    pub query: String,
    // Additional input fields for NLP queries
}

#[derive(SimpleObject)]
pub struct FinancialRecommendation {
    pub user_id: ID,
    pub recommendations: Vec<String>,
    // Additional fields for financial recommendations
}
