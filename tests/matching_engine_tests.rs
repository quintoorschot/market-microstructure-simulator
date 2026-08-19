use market_microstructure_simulator::matching_engine::MatchingEngine;
use market_microstructure_simulator::order_book::OrderBook;
use market_microstructure_simulator::order::*;
use market_microstructure_simulator::trade::Trade;

// ==================== TRADE PROCESSING TESTS  ====================
#[test]
fn test_buy_order_matches_standing_sell_order() -> () {
    let mut matching_engine = MatchingEngine::new();

    matching_engine.submit_order(Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Sell,
        timestamp: 100,
    });

    let trades = matching_engine.submit_order(Order {
        id: 1,
        price: 10002,
        quantity: 10,
        side: Side::Buy,
        timestamp: 110,
    });

    let expected_trades = vec![
        Trade { incoming_order_id: 1, resting_order_id: 0, price: 10000, quantity: 10 },
    ];

    // Test if the trades is processed as expected.
    assert_eq!(trades, expected_trades);

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(Order {
        id: 0,
        price: 10000,
        quantity: 15,
        side: Side::Sell,
        timestamp: 100,
    });

    // Test if the resulting order book is what we expect.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn test_sell_order_matches_standing_buy_order() -> () {
    let mut matching_engine = MatchingEngine::new();
    
    matching_engine.submit_order(Order {
        id: 0,
        price: 9999,
        quantity: 25,
        side: Side::Sell,
        timestamp: 100,
    });

    let trades = matching_engine.submit_order(Order {
        id: 1,
        price: 10001,
        quantity: 10,
        side: Side::Buy,
        timestamp: 110,
    });

    let expected_trades = vec![
        Trade { incoming_order_id: 1, resting_order_id: 0, price: 9999, quantity: 10 },
    ];

    // Test if the trades is processed as expected.
    assert_eq!(trades, expected_trades);

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(Order {
        id: 0,
        price: 9999,
        quantity: 15,
        side: Side::Sell,
        timestamp: 100,
    });

    // Test if the resulting order book is what we expect.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}


// ==================== ORDER CANCEL TESTS  ====================
#[test]
fn cancel_single_standing_buy_order() -> () {
    let mut matching_engine = MatchingEngine::new();

    // Assumes submit_order works as intended.
    matching_engine.submit_order(Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Buy,
        timestamp: 100,
    });

    let cancel_result = matching_engine.cancel_order(0);

    // Check if cancellation was succesful.
    assert_eq!(cancel_result, true);

    let expected_orderbook = OrderBook::new();

    // Check if order got cancelled and removed from orderbook.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn cancel_single_standing_sell_order() -> () {
    let mut matching_engine = MatchingEngine::new();

    // Assumes submit_order works as intended.
    matching_engine.submit_order(Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Sell,
        timestamp: 100,
    });

    let cancel_result = matching_engine.cancel_order(0);

    // Check if cancellation was succesful.
    assert_eq!(cancel_result, true);

    let expected_orderbook = OrderBook::new();

    // Check if order got cancelled and removed from orderbook.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn cancel_target_order_from_multiple_orders() -> () {
    let mut matching_engine = MatchingEngine::new();

    let orders = [
        Order { id: 0, price: 10002, quantity: 15, side: Side::Buy, timestamp: 100 },
        Order { id: 1, price: 10000, quantity: 25, side: Side::Buy, timestamp: 110 },
        Order { id: 2, price: 10000, quantity: 40, side: Side::Buy, timestamp: 120 },
    ];

    for order in orders {
        // Assumes submit_order works as intended.
        matching_engine.submit_order(order);
    }

    let cancel_result = matching_engine.cancel_order(1);

    // Check if cancellation was succesful.
    assert_eq!(cancel_result, true);

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(*orders.get(0).unwrap());
    expected_orderbook.store_order(*orders.get(2).unwrap());

    // Check if order got cancelled and removed from orderbook.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}