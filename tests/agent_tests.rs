use market_microstructure_simulator::agents::noise_trader::NoiseTrader;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_noise_agent_seed_determinism(seed in 0u64..100) {
        let mut noise_agent_1 = NoiseTrader::new(0, seed, 10_000, 1000);
        let mut noise_agent_2 = NoiseTrader::new(0, seed, 10_000, 1000);

        // Noisy agents with the same seed should generate the same order.
        for iteration in 0..10 {
            prop_assert_eq!(
                noise_agent_1.generate_order(),
                noise_agent_2.generate_order(),
                "Noise agents diverged at iteration {} with seed {}.",
                iteration,
                seed,
            );
        }
    }
}