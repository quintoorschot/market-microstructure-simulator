use crate::{order::Order, simulator::clock::SimTime};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct EventKey {
    pub time: SimTime,
    pub sequence: u64,
}

#[derive(Debug)]
pub enum SimulationEvent {
    SubmitOrder(Order),
    CancelOrder(u64),
    ModifyOrder(u64, u64, u64),
}
