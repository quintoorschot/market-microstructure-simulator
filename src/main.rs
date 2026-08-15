mod order;
mod order_book;
mod matching_engine;
mod trade;

use order::*;
use order_book::*;
use matching_engine::*;

fn main() {

    let mut matching_engine = MatchingEngine::new();

    let order = Order {
        id: 1,
        price: 10000,
        quantity: 10,
        side: Side::Buy,
        timestamp: 100,
    };

    matching_engine.submit_order(order);

    println!("{}", matching_engine.orderbook);
}
