use core::fmt;
use std::collections::BTreeMap;
use crate::{order::{Order, Side}, trade::Trade};


/// Stores resting orders that provide liquidity for the matching engine.
///
/// The order book maintains buy and sell orders grouped by price level.
/// It is responsible for storing and retrieving resting liquidity.
#[derive(PartialEq, Debug)]
pub struct OrderBook {

    // Price => Orders at that price
    pub bids: BTreeMap<i64, Vec<Order>>,
    pub asks: BTreeMap<i64, Vec<Order>>,
}

impl OrderBook {

    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }


    /// Store a resting order in the order book.
    pub fn store_order(&mut self, order: Order) {

        match &order.side {
            Side::Buy => {
                self.bids
                    .entry(order.price)
                    .or_insert_with(Vec::new)
                    .push(order);
            }

            Side::Sell => {
                self.asks
                    .entry(order.price)
                    .or_insert_with(Vec::new)
                    .push(order);
            }
        }
    }

    /// Returns the best (highest) bid price if it exists, otherwise returns None.
    pub fn best_bid(&self) -> Option<&i64> {
        self.bids.keys().next_back()
    }

    /// Returns the best (lowest) ask price if it exists, otherwise returns None.
    pub fn best_ask(&self) -> Option<&i64> {
        self.asks.keys().next()
    }

    pub(crate) fn match_at_best(&mut self, incoming: &mut Order) -> Trade {
        match incoming.side {
            Side::Buy => self.match_against_asks(incoming),
            Side::Sell => self.match_against_bids(incoming),
        }
    }

    fn match_against_asks(&mut self, incoming: &mut Order) -> Trade {
        let price = self
            .best_ask()
            .copied()
            .expect("match_against_asks called without asks in the order book.");

        let (trade, remove_price_level) = {
            let queue = self
                .asks
                .get_mut(&price)
                .expect("Best ask price must exist in asks.");

            let resting = queue
                .first_mut()
                .expect("Price level must have at least one resting order.");

            let quantity = incoming.quantity.min(resting.quantity);

            incoming.quantity -= quantity;
            resting.quantity -= quantity;

            let trade = Trade {
                incoming_order_id: incoming.id,
                resting_order_id: resting.id,
                price,
                quantity,
            };

            // Remove 'resting' from queue when no longer needed.
            if queue[0].quantity == 0 {
                queue.remove(0);
            }

            let remove_price_level = queue.is_empty();

            (trade, remove_price_level)
        };

        if remove_price_level {
            self.asks.remove(&price);
        }

        trade
    }

    fn match_against_bids(&mut self, incoming: &mut Order) -> Trade {
        let price = self
            .best_bid()
            .copied()
            .expect("match_against_bids called without bids in the order book.");

        let (trade, remove_price_level) = {
            let queue = self
                .bids
                .get_mut(&price)
                .expect("Best bid price must exist in bids.");

            let resting = queue
                .first_mut()
                .expect("Price level must have at least one resting order.");

            let quantity = incoming.quantity.min(resting.quantity);

            incoming.quantity -= quantity;
            resting.quantity -= quantity;

            let trade = Trade {
                incoming_order_id: incoming.id,
                resting_order_id: resting.id,
                price,
                quantity,
            };

            // Remove 'resting' from queue when no longer needed.
            if queue[0].quantity == 0 {
                queue.remove(0);
            }

            let remove_price_level = queue.is_empty();

            (trade, remove_price_level)
        };

        if remove_price_level {
            self.bids.remove(&price);
        }

        trade
    }

}


fn write_side(
    f: &mut fmt::Formatter<'_>,
    side: &str,
    levels: &BTreeMap<i64, Vec<Order>>,
) -> fmt::Result {

    for (price, orders) in levels {
        let total_quantity: u64 = orders
            .iter()
            .map(|o| o.quantity)
            .sum();

        writeln!(
            f,
            "{} {:>8.2} | {} order(s) | qty {}",
            side,
            *price as f64 / 100.0,
            orders.len(),
            total_quantity,
        )?;
    }

    Ok(())
}


impl fmt::Display for OrderBook {
    
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {

        writeln!(f, "============== BIDS ==============")?;
        write_side(f, "BUY", &self.bids)?;

        writeln!(f, "\n============== ASKS ==============")?;
        write_side(f, "SELL", &self.asks)?;

        Ok(())
    }

}