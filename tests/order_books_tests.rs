use market_microstructure_simulator::order::*;
use market_microstructure_simulator::order_book::OrderBook;
use proptest::prelude::*;

// ==================== BEST PRICE TESTS  ====================
#[test]
fn test_best_price_empty_order_book() {
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

proptest! {
    #[test]
    fn test_best_prices_with_mixed_entries(
        orders in prop::collection::vec(
            // (price, is_buy)
            (1u64..1_000_000, any::<bool>()),
            1..100
        )
    ) {
        let mut orderbook = OrderBook::new();

        for (id, &(price, is_buy)) in orders.iter().enumerate() {
            orderbook.store_order(Order {
                id: id as u64,
                price,
                quantity: 25,
                side: if is_buy {
                    Side::Buy
                } else {
                    Side::Sell
                }
            });
        }
        
        let expected_best_bid = orders
                                    .iter()
                                    .filter(|(_, is_buy)| *is_buy)
                                    .map(|(price, _)| price)
                                    .max();
        assert_eq!(orderbook.best_bid(), expected_best_bid);

        let expected_best_ask = orders
                                    .iter()
                                    .filter(|(_, is_buy)| !*is_buy)
                                    .map(|(price, _)| price)
                                    .min();
        assert_eq!(orderbook.best_ask(), expected_best_ask);
    }
}

proptest! {
    #[test]
    fn test_best_ask_with_sell_entries_only(
        prices in prop::collection::vec(1u64..1_000_000, 1..100)
    ) {
        let mut orderbook = OrderBook::new();

        for (id, &price) in prices.iter().enumerate() {
            orderbook.store_order(Order { id: id as u64, price, quantity: 25, side: Side::Sell });
        }

        let expected_best_ask = prices.iter().copied().min();

        prop_assert_eq!(
            orderbook.best_ask().copied(),
            expected_best_ask
        );

        prop_assert!(orderbook.best_bid().is_none());
    }
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
