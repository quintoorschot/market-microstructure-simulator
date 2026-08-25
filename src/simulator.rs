use std::collections::BTreeMap;

use crate::{
    clock::SimTime, matching_engine::MatchingEngine, simulation_events::{EventKey, SimulationEvent::{self, CancelOrder, SubmitOrder}},
};

#[derive(Debug)]
pub struct Simulator {
    pub matching_engine: MatchingEngine,
    current_time: SimTime,
    queue: BTreeMap<EventKey, SimulationEvent>,
    next_sequence: u64,
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            matching_engine: MatchingEngine::new(),
            current_time: SimTime::ZERO,
            queue: BTreeMap::new(),
            next_sequence: 0,
        }
    }

    pub fn schedule(&mut self, time: SimTime, event: SimulationEvent) {
        let key = EventKey {
            time,
            sequence: self.next_sequence,
        };
        self.queue.insert(key, event);
        self.next_sequence += 1;
    }

    pub fn step(&mut self) {
        let current_event = self.queue.pop_first();
        println!("{:?}", current_event);

        if let Some((key, event)) = current_event {

            self.current_time = self.current_time.clone().max(key.time);

            match event {
                SimulationEvent::SubmitOrder(order) => { self.matching_engine.submit_order(order); },
                SimulationEvent::CancelOrder(id) => { self.matching_engine.cancel_order(id); },
            }

        } else {
            println!("SIMULATION FINISHED!");
            return;
        }
    }

    pub fn run(&mut self) {
        while !self.queue.is_empty() {
            self.step();
            self.run();
        }
    }
}
