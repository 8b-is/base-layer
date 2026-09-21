fn le16(v: u16) -> [u8; 2] {
    v.to_le_bytes()
}

fn le32(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

/// 16-bit PCM RIFF/WAVE writer. `samples` are interleaved by channel.
pub fn write_wav(
    path: &str,
    sample_rate: u32,
    channels: u16,
    samples: &[i16],
) -> std::io::Result<()> {
    let bits = 16u16;
    let block_align = channels * (bits / 8);
    let byte_rate = sample_rate * block_align as u32;
    let data_len = (samples.len() * 2) as u32;

    let mut buf: Vec<u8> = Vec::with_capacity(44 + data_len as usize);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&le32(36 + data_len));
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&le32(16));
    buf.extend_from_slice(&le16(1));
    buf.extend_from_slice(&le16(channels));
    buf.extend_from_slice(&le32(sample_rate));
    buf.extend_from_slice(&le32(byte_rate));
    buf.extend_from_slice(&le16(block_align));
    buf.extend_from_slice(&le16(bits));
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&le32(data_len));
    for s in samples {
        buf.extend_from_slice(&s.to_le_bytes());
    }
    std::fs::write(path, buf)
}
