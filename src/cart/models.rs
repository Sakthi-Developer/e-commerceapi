
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CartItem {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub added_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct CartWithItems {
    pub cart: Cart,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddItem{
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}
