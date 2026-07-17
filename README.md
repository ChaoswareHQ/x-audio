# x-audio

[![Crates.io Version](https://img.shields.io/crates/v/x-audio)](https://crates.io/crates/x-audio)
[![Crates.io Downloads](https://img.shields.io/crates/d/x-audio)](https://crates.io/crates/x-audio)
[![docs.rs](https://img.shields.io/docsrs/x-audio)](https://docs.rs/x-audio)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-1.97%2B-orange)](https://www.rust-lang.org)

A lightweight audio output library written in Rust, designed to run
everywhere — from microcontrollers to modern desktops. Takes mono
`f32` sample buffers and plays them through the host audio device, or
buffers them for I2S DMA on embedded targets.

---

## Features

- **Dual backend** — host audio via `cpal` on desktop, sample buffer for MCU
- **Lock-free ring buffer** — writer (emulator) and reader (audio device) can run on different threads without contention
- **`no_std`-compatible** — the `mcu` backend has zero dependencies
- **Dual library output** — `lib`, `cdylib` (shared), and `staticlib` for flexible integration
- **Tiny footprint** — ~120 KB shared library (fully stripped)

## Building

```sh
# Default desktop build (cpal output)
cargo build --release

# MCU build (sample buffer only)
cargo build --release --no-default-features --features mcu

# Example: buffer audio frames headlessly
cargo run --release --example main -- "your-rom.nes"

# Example: MCU audio buffer test
cargo run --release --features mcu --example mcu -- "your-rom.nes"
```

## Usage

### As a Rust crate (via crates.io)

```toml
[dependencies]
x-audio = "0.1"
```

### Desktop audio playback

```rust
use audio::desktop::DesktopAudio;
use audio::output::AudioOut;

let mut audio = DesktopAudio::new(44100).expect("audio device");
audio.write(&apu_samples);
```

### MCU audio buffer

```rust
use audio::mcu::McuAudio;
use audio::output::AudioOut;

let mut audio = McuAudio::new(44100);
audio.write(&apu_samples);

// On a real MCU, read via I2S DMA:
while let Some(sample) = audio.read() {
    // send to DAC
}
```

## Project Structure

| Module | Description |
|--------|-------------|
| `buffer` | Lock-free ring buffer for mono `f32` samples |
| `output` | `AudioOut` trait + error types |
| `desktop` | Host audio via `cpal` (requires `std` feature) |
| `mcu` | Sample buffer for I2S DMA (requires `mcu` feature) |

## License

MIT OR Apache-2.0
