mod order;

use order::*;

fn main() {

    let order = Order {
        id: 1,
        price: 10000,
        quantity: 10,
        side: Side::Buy,
        timestamp: 100,
    };

    println!("{}", order);
}
