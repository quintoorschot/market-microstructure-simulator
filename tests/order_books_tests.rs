use market_microstructure_simulator::order_book::OrderBook;
use market_microstructure_simulator::order::*;


// ==================== BEST BID TESTS  ====================
#[test]
fn test_best_bid_empty() -> () {
    let orderbook = OrderBook::new();
    assert_eq!(orderbook.best_bid(), None);
}

#[test]
fn test_best_bid_with_entries() -> () {
    let mut orderbook = OrderBook::new();

    // (id, price)
    let orders = [
        (1, 10000),
        (2, 10002),
        (3, 9999),
    ];

    for (id, price) in orders {
        orderbook.store_order(Order {
            id,
            price,
            quantity: 25,
            side: Side::Buy,
            timestamp: id * 20
        });
    }

    assert_eq!(orderbook.best_bid(), Some(&10002));
}


// ==================== BEST BID TESTS  ====================
#[test]
fn test_best_ask_empty() -> () {
    let notebook = OrderBook::new();
    assert_eq!(notebook.best_ask(), None);
}

#[test]
fn test_best_ask_with_entries() -> () {
    let mut orderbook = OrderBook::new();

    // (id, price)
    let orders = [
        (1, 10000),
        (2, 10002),
        (3, 9999),
    ];

    for (id, price) in orders {
        orderbook.store_order(Order {
            id,
            price,
            quantity: 25,
            side: Side::Sell,
            timestamp: id * 20
        });
    }

    assert_eq!(orderbook.best_ask(), Some(&9999));
}