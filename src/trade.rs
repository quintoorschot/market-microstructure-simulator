#[derive(Debug, PartialEq)]
pub struct Trade {
    pub incoming_order_id: u64,
    pub resting_order_id: u64,
    pub price: i64,
    pub quantity: u64
}