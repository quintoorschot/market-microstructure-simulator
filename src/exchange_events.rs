/// Represents events that occurred because the matching engine processed the requests.
#[derive(Debug, PartialEq)]
pub enum ExchangeEvent {
    /// Represents a succesful trade execution.
    TradeExecuted {
        incoming_order_id: u64,
        resting_order_id: u64,
        price: u64,
        quantity: u64,

        incoming_remaining: u64,
        resting_remaining: u64,
    },

    /// Represents successful order cancellation.
    OrderCancelled { order_id: u64 },

    /// Represents failed order cancellation.
    CancellationRejected { order_id: u64 },

    /// Represents successful order modification.
    OrderModified {
        order_id: u64,
        old_price: u64,
        new_price: u64,
        old_quantity: u64,
        new_quantity: u64,
    },

    /// Represents failed order modification.
    ModificationFailed { order_id: u64 },
}
