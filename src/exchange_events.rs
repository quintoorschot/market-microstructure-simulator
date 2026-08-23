/// Represents events emitted by the matching engine in response to requests.
#[derive(Debug)]
pub enum ExchangeEvent {

    /// Represents successful order cancellation.
    OrderCancelled {
        order_id: u64,
    },

    /// Represents failed order cancellation.
    CancellationRejected {
        order_id: u64,
    },

    /// Represents successful order modification.
    OrderModified {
        order_id: u64,
        old_price: i64,
        new_price: i64,
        old_quantity: i64,
        new_quantity: i64,
    },

    /// Represents failed order modification.
    ModificationFailed {
        order_id: u64,
    }
}