use market_microstructure_simulator::exchange_events::ExchangeEvent;
use market_microstructure_simulator::matching_engine::MatchingEngine;
use market_microstructure_simulator::order::*;
use market_microstructure_simulator::order_book::OrderBook;
use market_microstructure_simulator::trade::Trade;

// ==================== TRADE PROCESSING TESTS  ====================
#[test]
fn test_buy_order_matches_standing_sell_order() {
    let mut matching_engine = MatchingEngine::new();

    matching_engine.submit_order(Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Sell,
    });

    let trades = matching_engine.submit_order(Order {
        id: 1,
        price: 10002,
        quantity: 10,
        side: Side::Buy,
    });

    let expected_trades = vec![Trade {
        incoming_order_id: 1,
        resting_order_id: 0,
        price: 10000,
        quantity: 10,
    }];

    // Test if the trades is processed as expected.
    assert_eq!(trades, expected_trades);

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(Order {
        id: 0,
        price: 10000,
        quantity: 15,
        side: Side::Sell,
    });

    // Test if the resulting order book is what we expect.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn test_sell_order_matches_standing_buy_order() {
    let mut matching_engine = MatchingEngine::new();

    matching_engine.submit_order(Order {
        id: 0,
        price: 9999,
        quantity: 25,
        side: Side::Sell,
    });

    let trades = matching_engine.submit_order(Order {
        id: 1,
        price: 10001,
        quantity: 10,
        side: Side::Buy,
    });

    let expected_trades = vec![Trade {
        incoming_order_id: 1,
        resting_order_id: 0,
        price: 9999,
        quantity: 10,
    }];

    // Test if the trades is processed as expected.
    assert_eq!(trades, expected_trades);

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(Order {
        id: 0,
        price: 9999,
        quantity: 15,
        side: Side::Sell,
    });

    // Test if the resulting order book is what we expect.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

// ==================== ORDER CANCEL TESTS  ====================
#[test]
fn test_cancel_single_standing_buy_order() {
    let mut matching_engine = MatchingEngine::new();

    // Assumes submit_order works as intended.
    matching_engine.submit_order(Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Buy,
    });

    let cancel_result = matching_engine.cancel_order(0);

    // Check if cancellation was succesful.
    assert!(matches!(
        cancel_result,
        ExchangeEvent::OrderCancelled { order_id: 0 }
    ));

    let expected_orderbook = OrderBook::new();

    // Check if order got cancelled and removed from orderbook.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn test_cancel_single_standing_sell_order() {
    let mut matching_engine = MatchingEngine::new();

    // Assumes submit_order works as intended.
    matching_engine.submit_order(Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Sell,
    });

    let cancel_result = matching_engine.cancel_order(0);

    // Check if cancellation was succesful.
    assert!(matches!(
        cancel_result,
        ExchangeEvent::OrderCancelled {order_id: 0}
    ));

    let expected_orderbook = OrderBook::new();

    // Check if order got cancelled and removed from orderbook.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn test_cancel_target_order_from_multiple_orders() {
    let mut matching_engine = MatchingEngine::new();

    let orders = [
        Order {
            id: 0,
            price: 10002,
            quantity: 15,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10000,
            quantity: 25,
            side: Side::Buy,
        },
        Order {
            id: 2,
            price: 10000,
            quantity: 40,
            side: Side::Buy,
        },
    ];

    // Submit each order to matching engine.
    orders.into_iter().for_each(|order| {
        matching_engine.submit_order(order);
    });

    let cancel_result = matching_engine.cancel_order(1);

    // Check if cancellation was succesful.
    assert!(matches!(
        cancel_result,
        ExchangeEvent::OrderCancelled { order_id: 1 }
    ));

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(orders[0]);
    expected_orderbook.store_order(orders[2]);

    // Check if order got cancelled and removed from orderbook.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn test_cancel_nonexisting_order() {
    let mut matching_engine = MatchingEngine::new();

    let order = Order {
        id: 0,
        price: 10000,
        quantity: 25,
        side: Side::Sell,
    };

    // Assumes submit_order works as intended.
    matching_engine.submit_order(order);

    let cancel_result = matching_engine.cancel_order(1);

    // Non-existing order cancellation should return a fail.
    assert!(matches!(
            cancel_result,
            ExchangeEvent::CancelRejected { order_id: 1 }
        ));

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(order);

    // Existing non-matching orders should not be removed.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

// ==================== ORDER MODIFY TESTS  ====================
#[test]
fn test_modify_order_to_same_price_same_quantity() {
    // Same price, same quantity -> nothing changes (keep queue position)
    let mut matching_engine = MatchingEngine::new();

    let orders = [
        Order {
            id: 0,
            price: 10000,
            quantity: 15,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10000,
            quantity: 25,
            side: Side::Buy,
        },
    ];

    orders
        .into_iter()
        .for_each(
            |order| { matching_engine.submit_order(order); }
        );

    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(orders[0]);
    expected_orderbook.store_order(orders[1]);

    // Test if initial priority queue is correctly ordered
    assert_eq!(matching_engine.orderbook, expected_orderbook);

    // Modify order 0 to keep the exact same values. Priority position and order values should be preserved.
    matching_engine.modify_order(orders[0].id, orders[0].price, orders[0].quantity);

    // Test if priority queue and orders remained unchanged.
    assert_eq!(matching_engine.orderbook, expected_orderbook);
}

#[test]
fn test_modify_order_to_same_price_lower_quantity() {
    // Same price, lower quantity -> keep queue position
    let mut matching_engine = MatchingEngine::new();

    // Here, order 0 will keep the same price (10000), but get a lower quantity (15 -> 5).
    let modify_order_to = (0, 10000, 5);

    let orders = [
        Order {
            id: 0,
            price: 10000,
            quantity: 15,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10000,
            quantity: 25,
            side: Side::Buy,
        },
    ];

    // Submit each order to matching engine.
    orders
        .into_iter()
        .for_each(|order| {
            matching_engine.submit_order(order);
        });

    // Test if initial priority queue is correctly ordered (price-time priority).
    {
        let mut expected_orderbook_before = OrderBook::new();
        expected_orderbook_before.store_order(orders[0]);
        expected_orderbook_before.store_order(orders[1]);

        assert_eq!(matching_engine.orderbook, expected_orderbook_before);
    }

    // Test if after the order modification, the values changed and queue position is preserved.
    {
        // Modify the order.
        matching_engine.modify_order(modify_order_to.0, modify_order_to.1, modify_order_to.2);

        // Modified order should have the lower quantity, same price, and same queue position.
        let mut expected_orderbook_after = OrderBook::new();

        // Order 0 should remain first in the priority queue and have the updates values.
        expected_orderbook_after.store_order(Order {
            id: modify_order_to.0,
            price: modify_order_to.1,
            quantity: modify_order_to.2,
            side: Side::Buy,
        });
        // Order 1 shouldn't change
        expected_orderbook_after.store_order(orders[1]);

        assert_eq!(matching_engine.orderbook, expected_orderbook_after);
    }
}

#[test]
fn test_modify_order_to_same_price_higher_quantity() {
    // Same price, higher quantity -> stay in same price level, back to start of queue
    let mut matching_engine = MatchingEngine::new();

    // Here, order 0 will keep the same price (10000), but get a higher quantity (15 -> 25).
    let modify_order_to = (0, 10000, 25);

    let orders = [
        Order {
            id: 0,
            price: 10000,
            quantity: 15,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10000,
            quantity: 25,
            side: Side::Buy,
        },
    ];

    // Submit each order to matching engine.
    orders
        .into_iter()
        .for_each(|order| {
            matching_engine.submit_order(order);
        });

    // Test if initial priority queue is correctly ordered (price-time priority).
    {
        let mut expected_orderbook_before = OrderBook::new();
        expected_orderbook_before.store_order(orders[0]);
        expected_orderbook_before.store_order(orders[1]);

        assert_eq!(matching_engine.orderbook, expected_orderbook_before);
    }

    // Test if after the order modification, the values changed and queue position is reverted.
    {
        // Modify the order.
        matching_engine.modify_order(modify_order_to.0, modify_order_to.1, modify_order_to.2);

        // Modified order should have the lower quantity, same price, and same queue position.
        let mut expected_orderbook_after = OrderBook::new();

        // Order 0 should remain remain in the same price level and move to the back of the priority queue.
        expected_orderbook_after.store_order(orders[1]);
        expected_orderbook_after.store_order(Order {
            id: modify_order_to.0,
            price: modify_order_to.1,
            quantity: modify_order_to.2,
            side: Side::Buy,
        });

        assert_eq!(matching_engine.orderbook, expected_orderbook_after);
    }
}

#[test]
fn test_modify_order_to_higher_price_same_quantity() {
    // Higher price, same quantity -> Move to different price level, and back to start of queue
    let mut matching_engine = MatchingEngine::new();

    // Here, order 0 will get a higher price (10000 -> 10005), but keep the same quantity (15).
    let modify_order_to = (0, 10005, 15);

    let orders = [
        Order {
            id: 0,
            price: 10000,
            quantity: 15,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10000,
            quantity: 10,
            side: Side::Buy,
        },
        Order {
            id: 2,
            price: 10005,
            quantity: 25,
            side: Side::Buy,
        },
    ];

    // Submit each order to matching engine.
    orders
        .into_iter()
        .for_each(|order| {
            matching_engine.submit_order(order);
        });

    // Test if initial priority queue is correctly ordered (price-time priority).
    {
        let mut expected_orderbook_before = OrderBook::new();
        expected_orderbook_before.store_order(orders[0]);
        expected_orderbook_before.store_order(orders[1]);
        expected_orderbook_before.store_order(orders[2]);

        assert_eq!(matching_engine.orderbook, expected_orderbook_before);
    }

        // Test if after the order modification, the values changed and queue position is preserved.
    {
        // Modify the order.
        matching_engine.modify_order(modify_order_to.0, modify_order_to.1, modify_order_to.2);

        // Modified order should have the lower quantity, same price, and same queue position.
        let mut expected_orderbook_after = OrderBook::new();

        // Order 0 should move to another price level and move to the back of the priority queue.
        expected_orderbook_after.store_order(orders[1]);
        expected_orderbook_after.store_order(orders[2]);
        expected_orderbook_after.store_order(Order {
            id: modify_order_to.0,
            price: modify_order_to.1,
            quantity: modify_order_to.2,
            side: Side::Buy,
        });

        assert_eq!(matching_engine.orderbook, expected_orderbook_after);
    }
}

#[test]
fn test_modify_order_unknown_id() {
    let mut matching_engine = MatchingEngine::new();

    matching_engine.submit_order(Order { id: 0, price: 10000, quantity: 15, side: Side::Buy });
    
    // Try to modify order that doesn't exist -> Should return `false`.
    let result = matching_engine.modify_order(1, 10002, 20);

    assert!(matches!(result, ExchangeEvent::ModificationFailed { order_id: 1 }));
}