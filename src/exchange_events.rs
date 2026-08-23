pub enum ExchangeEvent {

    OrderCancelled {
        order_id: u64,
    },

    CancelRejected {
        order_id: u64,
    }
}