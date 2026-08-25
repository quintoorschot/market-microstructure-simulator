use market_microstructure_simulator::order::*;
use market_microstructure_simulator::order_book::OrderBook;
use proptest::prelude::*;

// ==================== BEST BID TESTS  ====================
#[test]
fn test_best_order_book_empty() {
    let orderbook = OrderBook::new();
    assert!(orderbook.best_bid().is_none());
    assert!(orderbook.best_ask().is_none());
}

proptest! {
    #[test]
    fn test_best_bid_with_buy_entries_only(
        prices in prop::collection::vec(1u64..1_000_000, 1..100)
    ) {
        let mut orderbook = OrderBook::new();

        for (id, &price) in prices.iter().enumerate() {
            orderbook.store_order(Order { id: id as u64, price, quantity: 25, side: Side::Buy });
        }

        let expected_best_bid = prices.iter().copied().max();

        prop_assert_eq!(
            orderbook.best_bid().copied(),
            expected_best_bid
        );

        prop_assert!(orderbook.best_ask().is_none());
    }
}

#[test]
fn test_best_bid_with_mixed_entries() {
    let mut orderbook = OrderBook::new();

    // (price, side)
    let orders = [
        (10000, Side::Sell),
        (9999, Side::Buy),
        (10002, Side::Sell),
        (9998, Side::Buy),
    ];

    for (id, (price, side)) in orders.into_iter().enumerate() {
        orderbook.store_order(Order {
            id: id as u64,
            price,
            quantity: 25,
            side,
        });
    }

    assert_eq!(orderbook.best_bid(), Some(&9999));
}

// ==================== BEST ASK TESTS  ====================
#[test]
fn test_best_ask_empty() {
    let notebook = OrderBook::new();
    assert!(notebook.best_ask().is_none());
}

#[test]
fn test_best_ask_with_sell_entries_only() {
    let mut orderbook = OrderBook::new();

    // (id, price)
    let orders = [(1, 10000), (2, 10002), (3, 9999)];

    for (id, price) in orders {
        orderbook.store_order(Order {
            id,
            price,
            quantity: 25,
            side: Side::Sell,
        });
    }

    assert_eq!(orderbook.best_ask(), Some(&9999));
}

#[test]
fn test_best_ask_with_mixed_entries() {
    let mut orderbook = OrderBook::new();

    // (price, side)
    let orders = [
        (10000, Side::Sell),
        (9999, Side::Buy),
        (10002, Side::Sell),
        (9998, Side::Buy),
    ];

    for (id, (price, side)) in orders.into_iter().enumerate() {
        orderbook.store_order(Order {
            id: id as u64,
            price,
            quantity: 25,
            side,
        });
    }

    assert_eq!(orderbook.best_ask(), Some(&10000));
}

// ==================== STORE ORDER TESTS  ====================
#[test]
fn test_store_buy_order_as_bid() {
    let mut orderbook = OrderBook::new();

    orderbook.store_order(Order {
        id: 1,
        price: 10000,
        quantity: 25,
        side: Side::Buy,
    });

    assert_eq!(orderbook.best_bid(), Some(&10000));
    assert!(orderbook.best_ask().is_none());
}

#[test]
fn test_store_sell_order_as_ask() {
    let mut orderbook = OrderBook::new();

    orderbook.store_order(Order {
        id: 1,
        price: 10000,
        quantity: 25,
        side: Side::Sell,
    });

    assert_eq!(orderbook.best_ask(), Some(&10000));
    assert!(orderbook.best_bid().is_none());
}
