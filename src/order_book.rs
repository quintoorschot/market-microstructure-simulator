use crate::{
    order::{Order, Side},
    trade::Trade,
    exchange_events::ExchangeEvent,
};
use core::fmt;
use std::{collections::BTreeMap};

/// Represents the location of an order in the order book (side -> price level -> index within price level)
#[derive(Debug)]
struct OrderLocation {
    side: Side,
    price: i64,
    index: usize,
}

/// Stores resting orders that provide liquidity for the matching engine.
///
/// The order book maintains buy and sell orders grouped by price level.
/// It is responsible for storing and retrieving resting liquidity.
#[derive(PartialEq, Debug, Default)]
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
                    .or_default()
                    .push(order);
            }

            Side::Sell => {
                self.asks
                    .entry(order.price)
                    .or_default()
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

    fn find_order(&self, id: u64) -> Option<OrderLocation> {
        if let Some((&price, index)) = self.bids.iter().find_map(|(price, orders)| {
            orders
                .iter()
                .position(|order| order.id == id)
                .map(|index| (price, index))
        }) {
            return Some(OrderLocation {
                side: Side::Buy,
                price,
                index,
            });
        }

        if let Some((&price, index)) = self.asks.iter().find_map(|(price, orders)| {
            orders
                .iter()
                .position(|order| order.id == id)
                .map(|index| (price, index))
        }) {
            return Some(OrderLocation {
                side: Side::Sell,
                price,
                index,
            });
        }

        None
    }

    pub fn cancel_order(&mut self, id: u64) -> ExchangeEvent {
        let Some(location) = self.find_order(id) else {
            return ExchangeEvent::CancelRejected {
                order_id: id
            };
        };

        let book = match location.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let should_remove_price = {
            let orders = book.get_mut(&location.price).unwrap();
            orders.remove(location.index);
            orders.is_empty()
        };

        if should_remove_price {
            book.remove(&location.price);
        }

        ExchangeEvent::OrderCancelled {
            order_id: id
        }
    }

    pub fn modify_order(&mut self, id: u64, new_quantity: u64, new_price: i64) -> bool {

        let Some(location) = self.find_order(id) else {
            return false;
        };

        let book = match location.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let orders = book.get_mut(&location.price).unwrap();

        let keep_priority = {
            let order = &orders[location.index];
            new_price == order.price && new_quantity <= order.quantity
        };

        if keep_priority {
            let order = &mut orders[location.index];
            order.quantity = new_quantity;
        } else {
            let mut order = orders.remove(location.index);

            if orders.is_empty() {
                book.remove(&location.price);
            }

            order.price = new_price;
            order.quantity = new_quantity;

            book.entry(new_price)
                .or_default()
                .push(order);
        }

        true
    }
}

fn write_side(
    f: &mut fmt::Formatter<'_>,
    side: &str,
    levels: &BTreeMap<i64, Vec<Order>>,
) -> fmt::Result {
    for (price, orders) in levels {
        let total_quantity: u64 = orders.iter().map(|o| o.quantity).sum();

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "============== BIDS ==============")?;
        write_side(f, "BUY", &self.bids)?;

        writeln!(f, "\n============== ASKS ==============")?;
        write_side(f, "SELL", &self.asks)?;

        Ok(())
    }
}
