#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct SimTime(pub u64);

/// Represents simulated time in discrete-event simulation.
impl SimTime {
    pub const ZERO: Self = Self(0);

    /// Returns the currently simulated time in nano seconds.
    pub fn nanos(&self) -> u64 {
        self.0
    }

    /// Add nano seconds to current simulated time.
    pub fn add_nanos(self, nanos: u64) -> Self {
        Self(self.0.checked_add(nanos).expect("Simulation time overflow"))
    }
}
