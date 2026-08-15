mod order;
mod order_book;

use order::*;
use order_book::*;

fn main() {

    let mut orderbook = OrderBook::new();

    let order = Order {
        id: 1,
        price: 10000,
        quantity: 10,
        side: Side::Buy,
        timestamp: 100,
    };

    orderbook.add_order(order);

    println!("{}", orderbook);
}
