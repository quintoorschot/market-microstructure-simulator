mod order;
mod order_book;
mod matching_engine;
mod trade;

use order::*;
use order_book::*;
use matching_engine::*;

fn main() {

    let mut matching_engine = MatchingEngine::new();

    let order_1 = Order {
        id: 1,
        price: 10000,
        quantity: 10,
        side: Side::Sell,
        timestamp: 100,
    };

    let order_2 = Order {
        id: 2,
        price: 10002,
        quantity: 5,
        side: Side::Buy,
        timestamp: 110,
    };


    matching_engine.submit_order(order_1);
    println!("Order 1 submitted!");
    matching_engine.submit_order(order_2);
    println!("Order 2 submitted!");

    matching_engine.display_order_book();
}
