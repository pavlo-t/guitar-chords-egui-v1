use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;
use std::collections::VecDeque;
use std::f32::consts::PI;

pub fn play_frequency_sine_wave(frequency: f32) -> cpal::Stream {
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

pub fn play_frequency_sine_plus_harmonics(frequency: f32) -> cpal::Stream {
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
                    let base = 2.0 * PI * frequency * sample_clock / sample_rate;
                    *sample = volume * (1.0 * base.sin() + 0.5 * (2.0 * base).sin() + 0.25 * (3.0 * base).sin());
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

pub fn play_frequency_sawtooth(frequency: f32) -> cpal::Stream {
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
                    let phase = (sample_clock * frequency) / sample_rate;
                    *sample = volume * (2.0 * (phase % 1.0) - 1.0);
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

pub fn playe_frequency_karplus_strong(frequency: f32) -> cpal::Stream {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate().0 as f32;

    let mut plucked_string = PluckedString::new(frequency, sample_rate);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = plucked_string.next_sample();
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();
    stream
}

pub struct PluckedString {
    buffer: VecDeque<f32>,
    damping: f32,
}

impl PluckedString {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        let n_samples = (sample_rate / frequency).round() as usize;
        let mut rng = rand::thread_rng();

        let buffer = (0..n_samples).map(|_| rng.gen_range(-1.0..1.0)).collect();

        Self {
            buffer,
            damping: 0.996, // Adjust to control decay
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let first = self.buffer.pop_front().unwrap_or(0.0);
        let second = *self.buffer.front().unwrap_or(&0.0);
        let avg = self.damping * 0.5 * (first + second);
        self.buffer.push_back(avg);
        first
    }
}
