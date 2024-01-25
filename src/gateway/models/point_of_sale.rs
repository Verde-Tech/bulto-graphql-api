use async_graphql::*;

#[derive(SimpleObject)]
pub struct Sale {
    pub id: ID,
    pub user_id: ID,
    pub total_amount: f64,
    // Additional sale fields
}

#[derive(SimpleObject)]
pub struct InventoryItem {
    pub id: ID,
    pub name: String,
    pub quantity: i32,
    // Additional inventory item fields
}

#[derive(SimpleObject)]
pub struct POSResult {
    pub sale: Sale,
    pub inventory: Vec<InventoryItem>,
    // Additional result fields
}
