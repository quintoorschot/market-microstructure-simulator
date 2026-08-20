use crate::{order_book::OrderBook, trade::Trade};
use crate::order::{Order, Side};

pub struct MatchingEngine {
    pub orderbook: OrderBook,
}

impl MatchingEngine {

    pub fn new() -> Self {
        Self {
            orderbook: OrderBook::new(),
        }
    }

    pub fn submit_order(&mut self, mut order: Order) -> Vec<Trade> {

        let mut trades: Vec<Trade> = Vec::new();

        loop {

            // Nothing left to execute.
            if order.quantity <= 0 {
                break;
            };

            let opposite_price = match order.side {
                Side::Buy  => self.orderbook.best_ask(),
                Side::Sell => self.orderbook.best_bid(),
            };

            let Some(&best_price) = opposite_price else {
                break;
            };

            let crosses = match order.side {
                Side::Buy  => order.price >= best_price,
                Side::Sell => order.price <= best_price,
            };

            if !crosses {
                break;
            }

            let trade = self.orderbook.match_at_best(&mut order);

            trades.push(trade);
        }

        // An unfilled limit order becomes passive liquidity.
        if order.quantity > 0 {
            self.orderbook.store_order(order);
        }

        trades
    }


    /// Cancel standing order using the order's id.
    pub fn cancel_order(&mut self, id: u64) -> bool {
        self.orderbook.cancel_order(id)
    }

    pub fn modify_order(&mut self, id: u64, new_quantity: i64, new_price: u64) -> bool {
        self.orderbook.modify_order(id, new_quantity, new_price)
    }

    /// Display the matching engine's standing orders by printing the order book.
    pub fn display_order_book(&self) -> () {
        println!("{}", self.orderbook)
    }

}