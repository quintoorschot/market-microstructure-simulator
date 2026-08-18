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

    pub fn display_order_book(&self) -> () {
        println!("{}", self.orderbook)
    }

}