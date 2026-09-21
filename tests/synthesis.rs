use base_layer::Layer;

#[test]
fn renders_deterministically() {
    let a = base_layer::render(7, 8000, 1.0);
    let b = base_layer::render(7, 8000, 1.0);
    assert_eq!(a, b);
}

#[test]
fn length_matches() {
    let sr = 8000u32;
    let secs = 1.5f64;
    let s = base_layer::render(1, sr, secs);
    assert_eq!(s.len(), (sr as f64 * secs) as usize * 2);
}

#[test]
fn is_not_silent() {
    let s = base_layer::render(3, 8000, 2.0);
    let peak = s.iter().map(|x| x.abs()).max().unwrap();
    assert!(peak > 2000, "peak too quiet: {peak}");
}

#[test]
fn different_seeds_diverge() {
    let a = base_layer::render(11, 8000, 1.0);
    let b = base_layer::render(12, 8000, 1.0);
    assert_ne!(a, b);
}

#[test]
fn core1_reports_identity() {
    let c = base_layer::Core1;
    assert_eq!(c.core(), 1);
    assert!(c.name().contains("Kyuss"));
}

#[test]
fn wav_header_is_valid() {
    let sr = 8000u32;
    let s = base_layer::render(5, sr, 0.5);
    let path = std::env::temp_dir().join("base-layer-test.wav");
    let p = path.to_str().unwrap();
    base_layer::wav::write_wav(p, sr, 2, &s).unwrap();

    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WAVE");
    assert_eq!(&bytes[12..16], b"fmt ");
    let channels = u16::from_le_bytes([bytes[22], bytes[23]]);
    assert_eq!(channels, 2);
    let rate = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);
    assert_eq!(rate, sr);

    let _ = std::fs::remove_file(&path);
}
