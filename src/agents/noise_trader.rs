use crate::{
    agents::agent::Agent,
    order::{Order, Side},
    simulator::simulation_events::SimulationEvent,
};
use rand::{RngExt, SeedableRng, rngs::StdRng};

#[derive(Debug)]
pub struct NoiseTrader {
    id: u64,
    rng: StdRng,
    next_order_id: u64,
    reference_price: u64,
}

impl NoiseTrader {
    pub fn new(id: u64, seed: u64, reference_price: u64) -> Self {
        Self {
            id,
            rng: StdRng::seed_from_u64(seed),
            next_order_id: 0,
            reference_price,
        }
    }

    pub fn generate_order(&mut self) -> Order {
        let side = if self.rng.random_bool(0.5) {
            Side::Buy
        } else {
            Side::Sell
        };

        let quantity = self.rng.random_range(1..=10);
        let offset = self.rng.random_range(-5..=5);
        let price = self.reference_price.saturating_add_signed(offset).max(1);

        let order = Order {
            id: self.next_order_id,
            price,
            quantity,
            side,
        };

        self.next_order_id += 1;

        order
    }
}

impl Agent for NoiseTrader {
    fn id(&self) -> u64 {
        self.id
    }

    fn on_wakeup(
        &mut self,
        now: crate::simulator::clock::SimTime,
    ) -> Vec<(
        crate::simulator::clock::SimTime,
        crate::simulator::simulation_events::SimulationEvent,
    )> {
        let order = self.generate_order();
        vec![
            (now, SimulationEvent::SubmitOrder(order)),
            (now.add_nanos(10000000), SimulationEvent::AgentWake(self.id)),
        ]
    }
}
