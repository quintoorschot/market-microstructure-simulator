use std::collections::BTreeMap;

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

        if Self::cancel_on_side(&mut self.orderbook.bids, id) {
            return true;
        }
        Self::cancel_on_side(&mut self.orderbook.asks, id)
    }

    /// Auxiliary function of `cancel_order` to cancel order on a specific side of the order book.
    fn cancel_on_side(side: &mut BTreeMap<i64, Vec<Order>>, id: u64) -> bool {

        let (order_cancelled, empty_price) = 'search: {
            for (price, price_level) in side.iter_mut() {
                if let Some(index) = price_level.iter().position(|order| order.id == id) {
                    price_level.remove(index);

                    if price_level.is_empty() {
                        break 'search (true, Some(*price));
                    }
                    break 'search (true, None)
                }
            }

            break 'search (false, None)
        };

        if let Some(price) = empty_price {
            side.remove(&price);
        }

        order_cancelled
    }


    /// Display the matching engine's standing orders by printing the order book.
    pub fn display_order_book(&self) -> () {
        println!("{}", self.orderbook)
    }

}