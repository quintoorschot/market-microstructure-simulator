#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct SimTime(pub u64);

impl SimTime {
    pub const ZERO: Self = Self(0);

    pub fn nanos(&self) -> u64 {
        self.0
    }

    pub fn add_nanos(self, nanos: u64) -> Self {
        Self(self.0.checked_add(nanos).expect("Simulation time overflow"))
    }
}
