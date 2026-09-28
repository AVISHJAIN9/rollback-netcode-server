/// NTP-style clock offset and jitter estimator.
/// Complies with FR-012 and FR-013.
pub struct ClockEstimator {
    pub rtt_ms: f32,
    pub clock_offset_ms: f32,
    pub jitter_variance: f32,
}

impl ClockEstimator {
    pub fn new() -> Self {
        Self {
            rtt_ms: 0.0,
            clock_offset_ms: 0.0,
            jitter_variance: 0.0,
        }
    }

    pub fn update_probe(&mut self, t1: u64, t2: u64, t3: u64, t4: u64) {
        todo!("Calculate RTT and clock skew based on 4-timestamp probe compliant with FR-012")
    }
}
