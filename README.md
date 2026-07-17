# x-audio

A minimal audio output library, targeting everything from
microcontrollers to modern desktops.

Used by [x-chaos](https://github.com/ChaoswareHQ/x-chaos) for
retro-emulator audio, but general-purpose enough for any
application that needs cross-platform audio playback.

## Features

- **`std`** (default) — Desktop audio via `cpal` + lock-free ring buffer.
- **`mcu`** — Pure-data buffer (no hardware deps), ready for I2S DMA.

## Usage

```toml
[dependencies]
x-audio = "0.1"
```
