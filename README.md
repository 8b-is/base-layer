# base-layer

**core 1 of N** — the T0 default as a literal Rust synth.

A desert-sunset melodic house floor over a Kyuss doom underlay, rendered to
16-bit stereo PCM. Zero dependencies; the mechanism is plain arithmetic.

## Run

```bash
cargo run --release -- [seed] [out.wav]
# default: seed 0x8b15 -> base-layer-t0.wav, 118 BPM, 8 bars (~16.3s)
```

## Test

```bash
cargo test
```

## The layers

| core | name |
|------|------|
| 1 | desert-sunset melodic house + Kyuss floor |

cores 2..N plug the same `Layer` surface: `core()`, `name()`, `render(seed, sr, seconds)`.