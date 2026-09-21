//! base-layer — core 1 of N.
//!
//! The T0 default rendered as a literal synth: a desert-sunset melodic house
//! floor (four-on-the-floor kick, warm detuned pads, a pentatonic pluck) laid
//! over a Kyuss doom underlay (saturated sub-bass root). Zero dependencies;
//! the whole mechanism is plain arithmetic on PCM frames.

pub mod bitcoin;
pub mod dsp;
pub mod sha256;
pub mod wav;

use dsp::{Lfsr, OnePole};

pub const SAMPLE_RATE: u32 = 44_100;
pub const BPM: f64 = 118.0;
pub const BARS: usize = 8;

/// Whole bars only, so the render loops cleanly at the bar boundary.
pub fn default_seconds() -> f64 {
    seconds_for_bpm(BPM)
}

pub fn seconds_for_bpm(bpm: f64) -> f64 {
    BARS as f64 * 4.0 * 60.0 / bpm
}

/// The numbered layers that stack into the base layer. Core 1 is the T0
/// default; cores 2..N plug the same surface.
pub trait Layer {
    fn core(&self) -> usize;
    fn name(&self) -> &'static str;
    fn render(&self, seed: u64, sample_rate: u32, seconds: f64) -> Vec<i16>;
}

pub struct Core1;

impl Layer for Core1 {
    fn core(&self) -> usize {
        1
    }

    fn name(&self) -> &'static str {
        "desert-sunset melodic house + Kyuss floor"
    }

    fn render(&self, seed: u64, sample_rate: u32, seconds: f64) -> Vec<i16> {
        render(seed, sample_rate, seconds)
    }
}

const PAD_DETUNE: f64 = 0.003;

// D minor "sunset" progression: i - VI - III - VII (Dm, Bb, F, C).
const CHORDS: [[f64; 3]; 4] = [
    [50.0, 53.0, 57.0],
    [46.0, 53.0, 58.0],
    [41.0, 53.0, 60.0],
    [36.0, 55.0, 60.0],
];

// Bass roots follow the chords, drop-tuned toward D.
const BASS: [f64; 4] = [26.0, 34.0, 29.0, 36.0];

// D minor pentatonic pluck arch: D4 F4 G4 A4 C5 A4 G4 F4.
const LEAD: [f64; 8] = [62.0, 65.0, 67.0, 69.0, 72.0, 69.0, 67.0, 65.0];

/// Render `seconds` of stereo interleaved i16 PCM at `sample_rate`.
pub fn render(seed: u64, sample_rate: u32, seconds: f64) -> Vec<i16> {
    render_tuned(seed, sample_rate, seconds, BPM)
}

/// Render with an explicit tempo, so a block hash can drive the floor.
pub fn render_tuned(seed: u64, sample_rate: u32, seconds: f64, bpm: f64) -> Vec<i16> {
    const CH: usize = 2;
    let sr = sample_rate as f64;
    let total = (sr * seconds) as usize;
    let mut mix = vec![0f64; total * CH];
    let mut rng = Lfsr::new(seed);

    let spb = 60.0 / bpm;
    let sixteenth = spb / 4.0;
    let eighth = spb / 2.0;

    let mut bass_phase = 0.0f64;
    let mut pad_lp = OnePole::new();

    for i in 0..total {
        let t = i as f64 / sr;
        let beat = t / spb;
        let beat_i = beat.floor() as usize;
        let beat_p = beat - beat.floor();
        let bar_i = beat_i / 4;
        let t_in_beat = beat_p * spb;

        let kick = {
            let f = 55.0 + 105.0 * dsp::exp_decay(t_in_beat, 0.045);
            f64::sin(dsp::TAU * f * t_in_beat) * dsp::exp_decay(t_in_beat, 0.16)
        };

        let bass_f = dsp::midi_to_hz(BASS[bar_i % 4]);
        bass_phase += bass_f / sr;
        if bass_phase >= 1.0 {
            bass_phase -= 1.0;
        }
        let lope = 0.85 + 0.15 * f64::sin(dsp::TAU * t / 8.0);
        let bass = (dsp::soft_clip(dsp::saw(bass_phase), 1.7) * 0.55
            + f64::sin(dsp::TAU * bass_phase) * 0.55)
            * lope;

        let chord = CHORDS[bar_i % 4];
        let mut pad = 0.0;
        for n in chord {
            let f = dsp::midi_to_hz(n);
            pad += dsp::saw(f * t);
            pad += dsp::saw(f * (1.0 + PAD_DETUNE) * t) * 0.7;
        }
        let fade_in = (t / 1.5).min(1.0);
        let fade_out = ((seconds - t) / 1.5).clamp(0.0, 1.0);
        let pad = pad_lp.lowpass(pad, 0.12) * 0.10 * fade_in * fade_out;

        let sixth_i = (t / sixteenth).floor() as usize;
        let t_16 = t - sixth_i as f64 * sixteenth;
        let lf = dsp::midi_to_hz(LEAD[sixth_i % LEAD.len()]);
        let pluck = dsp::triangle(lf * t_16) * dsp::exp_decay(t_16, 0.085);
        let echo = if t_16 >= sixteenth {
            dsp::triangle(lf * (t_16 - sixteenth)) * dsp::exp_decay(t_16 - sixteenth, 0.085) * 0.45
        } else {
            0.0
        };
        let lead = pluck + echo;

        let eighth_i = (t / eighth).floor() as usize;
        let t_8 = t - eighth_i as f64 * eighth;
        let hat = if eighth_i % 2 == 1 {
            dsp::exp_decay(t_8, 0.03) * rng.bipolar()
        } else {
            0.0
        };

        let air = rng.bipolar() * 0.018;
        let air2 = rng.bipolar() * 0.018;

        mix[i * CH] = kick * 0.75 + bass * 0.22 + pad + lead * 0.16 + hat * 0.22 + air;
        mix[i * CH + 1] = kick * 0.75 + bass * 0.22 + pad + lead * 0.20 + hat * 0.15 + air2;
    }

    let mut out = Vec::with_capacity(mix.len());
    for s in mix {
        let v = (s * 0.9).tanh();
        out.push((v * 32767.0) as i16);
    }
    out
}
