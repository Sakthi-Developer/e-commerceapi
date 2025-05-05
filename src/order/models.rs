use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::NaiveDateTime;


#[derive(Deserialize, Serialize, Debug)]
pub struct CheckOut {
    pub(crate) cart_id: Uuid
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub payment_id: String,
    pub status: String,
    pub total_amount: i64,
    pub currency: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: i64,
}
