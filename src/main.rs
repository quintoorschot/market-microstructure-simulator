mod matching_engine;
mod order;
mod order_book;
mod trade;
mod exchange_events;

use exchange_events::*;
use matching_engine::*;
use order::*;

fn main() {
    let mut matching_engine = MatchingEngine::new();

    let order_1 = Order {
        id: 1,
        price: 10000,
        quantity: 15,
        side: Side::Sell,
    };

    let order_2 = Order {
        id: 2,
        price: 10000,
        quantity: 15,
        side: Side::Sell,
    };

    matching_engine.submit_order(order_1);
    println!("Order 1 submitted!");

    matching_engine.modify_order(100, 10005, 15);
    // matching_engine.submit_order(order_2);
    // println!("Order 2 submitted!");

    matching_engine.display_order_book();
}
