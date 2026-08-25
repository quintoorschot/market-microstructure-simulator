use crate::{clock::SimTime, order::Order};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct EventKey {
    pub time: SimTime,
    pub sequence: u64,
}

#[derive(Debug)]
pub enum SimulationEvent {
    SubmitOrder(Order),
    CancelOrder(u64),
}
