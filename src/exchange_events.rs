/// Represents events emitted by the matching engine in response to requests.
#[derive(Debug, PartialEq)]
pub enum ExchangeEvent {
    /// Represents a succesful trade execution.
    TradeExecuted {
        incoming_order_id: u64,
        resting_order_id: u64,
        price: i64,
        quantity: i64,

        incoming_remaining: i64,
        resting_remaining: i64,
    },

    /// Represents successful order cancellation.
    OrderCancelled { order_id: u64 },

    /// Represents failed order cancellation.
    CancellationRejected { order_id: u64 },

    /// Represents successful order modification.
    OrderModified {
        order_id: u64,
        old_price: i64,
        new_price: i64,
        old_quantity: i64,
        new_quantity: i64,
    },

    /// Represents failed order modification.
    ModificationFailed { order_id: u64 },
}
