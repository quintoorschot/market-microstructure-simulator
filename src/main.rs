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

    let noise_trader = NoiseTrader::new(0, 42, 10_000);

    simulator.add_agent(noise_trader, SimTime(1000));

    simulator.run();

    println!("{}", simulator.retrieve_order_book());
}
