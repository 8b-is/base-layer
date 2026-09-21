use base_layer::bitcoin;

#[test]
fn sha256_nist_vectors() {
    assert_eq!(
        base_layer::sha256::sha256(b""),
        hex("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    );
    assert_eq!(
        base_layer::sha256::sha256(b"abc"),
        hex("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
    );
}

#[test]
fn double_sha256_empty() {
    assert_eq!(
        base_layer::sha256::double_sha256(b""),
        hex("5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456")
    );
}

#[test]
fn genesis_block_hash_matches() {
    let header = bitcoin::from_hex(
        "010000000000000000000000000000000000000000000000000000000000000000000000\
         3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c",
    );
    let mut hdr = [0u8; 80];
    hdr.copy_from_slice(&header);
    let h = bitcoin::block_hash(&hdr);
    assert_eq!(
        bitcoin::to_hex(&h),
        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
    );
}

#[test]
fn mine_hits_the_target() {
    let prefix = bitcoin::demo_prefix();
    let (nonce, h) = bitcoin::mine(&prefix, 8, 0).expect("found under 8 bits");
    assert!(bitcoin::leading_zero_bits(&h) >= 8);
    assert!(nonce < 4_000_000);
}

#[test]
fn difficulty_slows_the_floor() {
    assert!(bitcoin::tempo_from_bits(0) < bitcoin::tempo_from_bits(20));
}

#[test]
fn hash_drives_a_synth_seed() {
    let prefix = bitcoin::demo_prefix();
    let (_, h) = bitcoin::mine(&prefix, 4, 0).unwrap();
    let seed = bitcoin::seed_from_hash(&h);
    assert_eq!(seed, bitcoin::seed_from_hash(&h));
    // a harder target lands a different nonce, hence a different hash/seed
    let (_, harder) = bitcoin::mine(&prefix, 6, 0).unwrap();
    assert_ne!(seed, bitcoin::seed_from_hash(&harder));
}

fn hex(s: &str) -> [u8; 32] {
    let v = bitcoin::from_hex(s);
    let mut out = [0u8; 32];
    out.copy_from_slice(&v);
    out
}
