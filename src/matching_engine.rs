use crate::{order_book::OrderBook, trade::Trade};
use crate::Order;


pub struct ExecutionResult {
    pub trades: Vec<Trade>,
    pub remaining_order: Option<Order>,
}

pub struct MatchingEngine {
    orderbook: OrderBook,
}

impl MatchingEngine {

    pub fn new() -> Self {
        Self {
            orderbook: OrderBook::new(),
        }
    }

    pub fn submit_order(&mut self, order: Order) -> ExecutionResult {

        let mut trades: Vec<Trade> = Vec::new();

        

        self.orderbook.store_order(order);

        ExecutionResult {
            trades,
            remaining_order: None,
        }
    }

    pub fn display_order_book(&self) -> () {
        println!("{}", self.orderbook)
    }

}