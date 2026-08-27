use std::collections::{BTreeMap, HashMap};

use crate::{
    agents::agent::Agent,
    matching_engine::MatchingEngine,
    order_book::OrderBook,
    simulator::{
        clock::SimTime,
        simulation_events::{EventKey, SimulationEvent},
    },
};

#[derive(Debug)]
pub struct Simulator {
    matching_engine: MatchingEngine,
    current_time: SimTime,
    queue: BTreeMap<EventKey, SimulationEvent>,
    next_sequence: u64,
    agents: HashMap<u64, Box<dyn Agent>>,
    time_limit: SimTime,
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            matching_engine: MatchingEngine::new(),
            current_time: SimTime::ZERO,
            queue: BTreeMap::new(),
            next_sequence: 0,
            agents: HashMap::new(),
            time_limit: SimTime(1_000_000_000),
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

    pub fn add_agent<A>(&mut self, agent: A, first_wake: SimTime)
    where
        A: Agent + 'static,
    {
        let id = agent.id();

        self.agents.insert(id, Box::new(agent));

        self.schedule(
            first_wake,
            SimulationEvent::AgentWake(id)
        );
    }

    pub fn step(&mut self) {
        let current_event = self.queue.pop_first();

        if let Some((key, event)) = current_event {
            self.current_time = self.current_time.max(key.time);

            match event {
                SimulationEvent::SubmitOrder(order) => {
                    self.matching_engine.submit_order(order);
                }
                SimulationEvent::CancelOrder(id) => {
                    self.matching_engine.cancel_order(id);
                }
                SimulationEvent::ModifyOrder(id, new_price, new_quantity) => {
                    self.matching_engine
                        .modify_order(id, new_price, new_quantity);
                }
                SimulationEvent::AgentWake(id) => {
                    let events = self.agents
                        .get_mut(&id)
                        .unwrap()
                        .on_wakeup(self.current_time);
                    for (schedule_time, event) in events.into_iter() {
                        self.schedule(schedule_time, event);
                    }
                }
            }
        }
    }

    pub fn retrieve_order_book(&self) -> &OrderBook {
        &self.matching_engine.orderbook
    }

    pub fn run(&mut self) {
        while !self.queue.is_empty() && self.current_time < self.time_limit {
            self.step();
        }
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}
