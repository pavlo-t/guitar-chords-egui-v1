use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

pub fn play_frequency(frequency: f32) -> cpal::Stream {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate().0 as f32;
    let mut sample_clock = 0f32;
    let volume = 0.5;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = volume * (2.0 * PI * frequency * sample_clock / sample_rate).sin();
                    sample_clock = (sample_clock + 1.0) % sample_rate;
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();
    stream
}
