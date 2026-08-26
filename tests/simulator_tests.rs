use market_microstructure_simulator::{
    order::{Order, Side},
    order_book::OrderBook,
    simulator::clock,
    simulator::simulation_events::SimulationEvent,
    simulator::simulator::Simulator,
};

#[test]
pub fn test_schedule_submit_orders() {
    let mut simulator = Simulator::new();

    // Define orders to be scheduled.
    let orders = [
        Order {
            id: 0,
            price: 10002,
            quantity: 25,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10000,
            quantity: 10,
            side: Side::Sell,
        },
        Order {
            id: 2,
            price: 9999,
            quantity: 5,
            side: Side::Sell,
        },
    ];

    // Schedule events in the simulator's queue.
    orders.into_iter().for_each(|order| {
        simulator.schedule(
            clock::SimTime(order.id * 20),
            SimulationEvent::SubmitOrder(order),
        );
    });

    // Simulator's order book should be empty before any events are ran.
    assert_eq!(*simulator.retrieve_order_book(), OrderBook::new());

    // Run the simulator until either the time limit is reached or the queue is empty.
    simulator.run();

    // After running the simulator, order book should contain remaining standing order.
    let mut expected_orderbook = OrderBook::new();
    expected_orderbook.store_order(Order {
        id: 0,
        price: 10002,
        quantity: 10,
        side: Side::Buy,
    });

    // Compare the simulator's order book afer the simulation with what is to be expected.
    assert_eq!(*simulator.retrieve_order_book(), expected_orderbook);
}

#[test]
pub fn test_schedule_cancel_partially_processed_order() {
    let mut simulator = Simulator::new();

    // Define orders to be scheduled.
    let orders = [
        Order {
            id: 0,
            price: 10002,
            quantity: 25,
            side: Side::Buy,
        },
        Order {
            id: 1,
            price: 10003,
            quantity: 20,
            side: Side::Buy,
        },
        Order {
            id: 2,
            price: 10000,
            quantity: 10,
            side: Side::Sell,
        },
    ];

    // Schedule submit order events in the simulator's queue.
    orders.into_iter().for_each(|order| {
        simulator.schedule(
            clock::SimTime(order.id * 20),
            SimulationEvent::SubmitOrder(order),
        );
    });

    // Schedule cancel order 1 in the simulator's queue.
    simulator.schedule(clock::SimTime(80), SimulationEvent::CancelOrder(1));

    // Simulator's order book should be empty before any events are ran.
    assert_eq!(*simulator.retrieve_order_book(), OrderBook::new());

    // Run the simulator until either the time limit is reached or the queue is empty.
    simulator.run();

    // Construct expected order book. Only order 0 should remain after simulation ran.
    let mut expected_order_book = OrderBook::new();
    expected_order_book.store_order(Order {
        id: 0,
        price: 10002,
        quantity: 25,
        side: Side::Buy,
    });

    assert_eq!(*simulator.retrieve_order_book(), expected_order_book);
}
