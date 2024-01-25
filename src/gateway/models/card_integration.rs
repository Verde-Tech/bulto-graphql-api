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

#[derive(Mutation)]
impl MutationRoot {
    async fn add_card(&self, ctx: &Context<'_>, input: CardInput) -> Result<CardResult> {
        // Logic to add a new card
        todo!()
    }

    async fn remove_card(&self, ctx: &Context<'_>, card_id: ID) -> Result<Boolean> {
        // Logic to remove an existing card
        todo!()
    }
}

#[derive(Query)]
impl QueryRoot {
    async fn get_cards(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<Card>> {
        // Logic to retrieve all cards for a user
        todo!()
    }

    async fn get_card(&self, ctx: &Context<'_>, card_id: ID) -> Result<Card> {
        // Logic to retrieve a specific card by ID
        todo!()
    }
}
