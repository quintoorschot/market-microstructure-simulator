use core::fmt;
use std::collections::BTreeMap;
use crate::order::{Side, Order};


/// Stores resting orders that provide liquidity for the matching engine.
///
/// The order book maintains buy and sell orders grouped by price level.
/// It is responsible for storing and retrieving resting liquidity.
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