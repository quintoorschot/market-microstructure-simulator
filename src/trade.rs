/// Represents an executed trade between an incoming order and a resting order.
///
/// ## Fields
///
/// - `incoming_order_id` (u64): ID of the order that triggered the match.
/// - `resting_order_id` (u64): ID of the order that provided liquidity.
/// - `price` (i64): Price at which the trade was executed.
/// - `quantity` (u64): Quantity executed between the two orders.
#[derive(Debug, PartialEq)]
pub struct Trade {
    /// ID of the order that triggered to match.
    pub incoming_order_id: u64,

    /// ID of the order that provided the liquidity.
    pub resting_order_id: u64,

    /// Price at which the trade was executed.
    pub price: i64,

    /// Quantity executed between the two orders.
    pub quantity: u64,
}
