use crate::simulator::{clock::SimTime, simulation_events::SimulationEvent};

pub trait Agent: std::fmt::Debug {
    fn id(&self) -> u64;

    fn on_wakeup(&mut self, now: SimTime) -> Vec<(SimTime, SimulationEvent)>;
}
