pub const TAU: f64 = std::f64::consts::TAU;

/// Deterministic xorshift64. Same seed, same stream.
pub struct Lfsr(u64);

impl Lfsr {
    pub fn new(seed: u64) -> Lfsr {
        Lfsr(seed.wrapping_add(0x9e37_79b9_7f4a_7c15).max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Uniform in [0, 1).
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0)
    }

    /// Uniform in [-1, 1).
    pub fn bipolar(&mut self) -> f64 {
        self.unit() * 2.0 - 1.0
    }
}

pub fn midi_to_hz(m: f64) -> f64 {
    440.0 * 2f64.powf((m - 69.0) / 12.0)
}

/// Rising saw, phase in [0, 1), out in [-1, 1].
pub fn saw(phase: f64) -> f64 {
    let p = phase - phase.floor();
    2.0 * p - 1.0
}

/// Triangle, phase in [0, 1), out in [-1, 1].
pub fn triangle(phase: f64) -> f64 {
    let p = phase - phase.floor();
    4.0 * (p - 0.5).abs() - 1.0
}

/// Exponential decay gate; `t` seconds since trigger, `tau` decay time.
pub fn exp_decay(t: f64, tau: f64) -> f64 {
    if t <= 0.0 {
        1.0
    } else {
        (-t / tau).exp()
    }
}

/// Tanh saturation; drive > 1 adds grit.
pub fn soft_clip(x: f64, drive: f64) -> f64 {
    (x * drive).tanh()
}

/// One-pole low/high pass pair.
pub struct OnePole {
    state: f64,
}

impl OnePole {
    pub fn new() -> OnePole {
        OnePole { state: 0.0 }
    }

    pub fn lowpass(&mut self, x: f64, alpha: f64) -> f64 {
        self.state += alpha * (x - self.state);
        self.state
    }

    pub fn highpass(&mut self, x: f64, alpha: f64) -> f64 {
        let lp = self.lowpass(x, alpha);
        x - lp
    }
}
