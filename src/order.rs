use core::fmt;

/// Represents the side (buy/sell) of an order.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

/// Represents an instruction to buy or sell an asset.
///
/// - `id` (u64): identifier for the order.
/// - `price` (i64): price in ticks (e.g. 10050 = $100.50) at which the order is willing to execute.
/// - `quantity` (u64): number of units the order is willing to buy or sell.
///  - `side` (Side): indicates whether the order is a buy or sell.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Order {
    /// Identifier for the order.
    pub id: u64,

    /// Price in ticks (e.g. 10050 = $100.50) at which the order is willing to execute.
    pub price: i64,

    /// Number of units the order is willing to buy or sell.
    pub quantity: u64,

    /// indicates whether the order is a buy or sell.
    pub side: Side,
}

impl Order {
    fn price_decimal(&self) -> f64 {
        self.price as f64 / 100.0
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = match self.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        write!(
            f,
            "{} {} @ {:.2} (order_id={})",
            side,
            self.quantity,
            self.price_decimal(),
            self.id,
        )
    }
}
