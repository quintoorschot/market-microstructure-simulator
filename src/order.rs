#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub price: i64,
    pub quantity: u64,
    pub side: Side,
}