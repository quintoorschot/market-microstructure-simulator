mod exchange_events;
mod matching_engine;
mod order;
mod order_book;
mod simulator;
mod trade;

use exchange_events::*;
use matching_engine::*;
use order::*;
use simulation_events::*;
use simulator::*;

fn main() {
    let mut simulator = simulator::Simulator::new();
    let mut matching_engine = MatchingEngine::new();

    let order_1 = Order {
        id: 1,
        price: 10000,
        quantity: 15,
        side: Side::Sell,
    };

    let order_2 = Order {
        id: 2,
        price: 10002,
        quantity: 10,
        side: Side::Sell,
    };

    simulator.schedule(clock::SimTime(100), SimulationEvent::SubmitOrder(order_1));
    simulator.schedule(clock::SimTime(110), SimulationEvent::SubmitOrder(order_2));
    simulator.schedule(
        clock::SimTime(120),
        SimulationEvent::ModifyOrder(1, 10003, 5),
    );

    // simulator.step();
    // simulator.step();
    simulator.run();

    let orderbook = simulator.retrieve_order_book();
    println!("{}", orderbook);

    // println!("{:?}", simulator);
}
