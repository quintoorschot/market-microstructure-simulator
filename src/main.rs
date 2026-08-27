mod agents;
mod exchange_events;
mod matching_engine;
mod order;
mod order_book;
mod simulator;
mod trade;

use agents::noise_trader;
use agents::*;
use exchange_events::*;
use matching_engine::*;
use order::*;
use simulation_events::*;
use simulator::*;

use crate::agents::noise_trader::NoiseTrader;
use crate::simulator::clock::SimTime;

fn main() {
    let mut simulator = simulator::Simulator::new();

    let mut noise_trader = NoiseTrader::new(42, 10_000);

    for i in 0..100 {
        let order = noise_trader.generate_order();

        simulator.schedule(SimTime(i * 1000), SimulationEvent::SubmitOrder(order));
    }

    simulator.run();

    println!("{}", simulator.retrieve_order_book());
}
