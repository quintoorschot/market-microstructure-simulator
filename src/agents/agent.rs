use crate::{
    order_book::OrderBook,
    simulator::{clock::SimTime, simulation_events::SimulationEvent},
};

pub trait Agent: std::fmt::Debug {
    fn id(&self) -> u64;

    fn on_wakeup(
        &mut self,
        now: SimTime,
        order_book: &OrderBook,
    ) -> Vec<(SimTime, SimulationEvent)>;
}
