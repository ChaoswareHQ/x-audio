/// MCU audio example: push APU samples through McuAudio.
///
/// Usage: cargo run --features mcu --example mcu -- <rom.nes> [frames]
use std::time::Instant;

use audio::mcu::McuAudio;
use audio::output::AudioOut;

use nes::bus::Bus;
use nes::cpu::CpuRp2a03;
use nes::rom::Rom;
use nes::{reset, tick};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom.nes> [frames]", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let max_frames: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60);

    let data = std::fs::read(path).expect("failed to read ROM");
    let rom = Rom::new(&data).expect("invalid iNES ROM");

    let mut cpu = CpuRp2a03::new(0);
    let mut bus = Bus::new(rom.create_mapper());
    reset(&mut cpu, &mut bus);

    let sample_rate = 44100;
    bus.apu.set_sample_rate(sample_rate as f64);

    let mut audio = McuAudio::new(sample_rate);
    let mut frame_count = 0u32;
    let mut total_samples = 0usize;
    let start = Instant::now();

    while frame_count < max_frames {
        while !bus.ppu.frame_complete {
            tick(&mut cpu, &mut bus);
        }
        bus.ppu.frame_complete = false;

        let n = bus.apu.sample_count;
        if n > 0 {
            let pushed = audio.write(&bus.apu.audio_samples[..n]);
            total_samples += pushed;
        }
        bus.apu.sample_count = 0;
        frame_count += 1;
    }

    let elapsed = start.elapsed();
    let fps = frame_count as f64 / elapsed.as_secs_f64();

    println!("Rendered {frame_count} frames from {path}");
    println!("  Sample rate: {sample_rate} Hz");
    println!("  Total samples: {total_samples}");
    println!("  Buffer available: {}", audio.read().is_some() as u8);
    println!("  Time: {:.3}s", elapsed.as_secs_f64());
    println!("  FPS:  {fps:.1}");
}
