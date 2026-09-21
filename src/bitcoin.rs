//! Bitcoin's base layer as a descriptor: the 0 (fixed header, found nonce) and
//! the 1 (one canonical double-SHA256 chain). No consensus, no networking —
//! just the mechanism a synth can listen to.

use crate::sha256::double_sha256;

pub const HEADER_LEN: usize = 80;

/// Hash a full 80-byte header, returned in Bitcoin's display order (the
/// big-endian digest, byte-reversed — the "000..." form you see on explorers).
pub fn block_hash(header: &[u8; HEADER_LEN]) -> [u8; 32] {
    let d = double_sha256(header);
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = d[31 - i];
    }
    out
}

/// Leading zero bits of a display-order hash. A simplified proof-of-work
/// metric, not the compact `bits` target inequality.
pub fn leading_zero_bits(h: &[u8; 32]) -> u32 {
    let mut n = 0u32;
    for &b in h {
        let lz = b.leading_zeros();
        n += lz;
        if lz < 8 {
            break;
        }
    }
    n
}

/// Find a nonce so `block_hash(prefix || nonce)` has at least `bits` leading
/// zero bits. Incrementing nonce, the true miner's move.
pub fn mine(prefix76: &[u8], bits: u32, start_nonce: u32) -> Option<(u32, [u8; 32])> {
    let mut hdr = [0u8; HEADER_LEN];
    hdr[..76].copy_from_slice(prefix76);
    let mut nonce = start_nonce;
    for _ in 0..4_000_000u32 {
        hdr[76..80].copy_from_slice(&nonce.to_le_bytes());
        let h = block_hash(&hdr);
        if leading_zero_bits(&h) >= bits {
            return Some((nonce, h));
        }
        nonce = nonce.wrapping_add(1);
    }
    None
}

/// The first 76 bytes of the genesis header, so mining re-proves genesis.
pub fn demo_prefix() -> Vec<u8> {
    from_hex(
        "010000000000000000000000000000000000000000000000000000000000000000000000\
         3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d",
    )
}

/// The found hash, folded into the synth's seed.
pub fn seed_from_hash(h: &[u8; 32]) -> u64 {
    u64::from_le_bytes([h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]])
}

/// Harder work, slower floor — the Kyuss doom deepens with difficulty.
pub fn tempo_from_bits(bits: u32) -> f64 {
    96.0 + bits as f64 * 4.0
}

pub fn to_hex(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for byte in b {
        s.push_str(&format!("{byte:02x}"));
    }
    s
}

pub fn from_hex(s: &str) -> Vec<u8> {
    let clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    (0..clean.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean[i..i + 2], 16).expect("bad hex"))
        .collect()
}
