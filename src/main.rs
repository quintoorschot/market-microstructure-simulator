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
        id: 1,
        price: 10002,
        quantity: 10,
        side: Side::Sell,
        timestamp: 100,
    };

    let order_3 = Order {
        id: 1,
        price: 9999,
        quantity: 10,
        side: Side::Sell,
        timestamp: 100,
    };


    // let order_2 = Order {
    //     id: 1,
    //     price: 10000,
    //     quantity: 8,
    //     side: Side::Sell,
    //     timestamp: 100,
    // };

    // matching_engine.submit_order(order_1);
    // matching_engine.submit_order(order_2);


    // matching_engine.display_order_book();



    let mut orderbook = OrderBook::new();
    orderbook.store_order(order_1);
    orderbook.store_order(order_2);
    orderbook.store_order(order_3);
    orderbook.best_ask();
}
