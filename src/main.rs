use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sr = base_layer::SAMPLE_RATE;

    if args.get(1).map(|a| a == "--prove").unwrap_or(false) {
        let bits: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(16);
        let out = args
            .get(3)
            .cloned()
            .unwrap_or_else(|| "base-layer-proven.wav".to_string());

        let prefix = base_layer::bitcoin::demo_prefix();
        match base_layer::bitcoin::mine(&prefix, bits, 0) {
            Some((nonce, hash)) => {
                let seed = base_layer::bitcoin::seed_from_hash(&hash);
                let bpm = base_layer::bitcoin::tempo_from_bits(bits);
                let seconds = base_layer::seconds_for_bpm(bpm);
                let samples = base_layer::render_tuned(seed, sr, seconds, bpm);
                base_layer::wav::write_wav(&out, sr, 2, &samples).expect("failed to write WAV");
                eprintln!(
                    "mined nonce {} ({} bits) -> {} -> seed {:#x}, bpm {:.1} -> {}",
                    nonce,
                    bits,
                    base_layer::bitcoin::to_hex(&hash),
                    seed,
                    bpm,
                    out
                );
            }
            None => eprintln!("no nonce found under {} bits in the budget", bits),
        }
        return;
    }

    let seed: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0x8b_15);
    let out = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "base-layer-t0.wav".to_string());

    let seconds = base_layer::default_seconds();
    let samples = base_layer::render(seed, sr, seconds);
    base_layer::wav::write_wav(&out, sr, 2, &samples).expect("failed to write WAV");

    eprintln!(
        "core 1 of N -> {} ({}s, {} frames, {} Hz)",
        out,
        seconds,
        samples.len() / 2,
        sr
    );
}
