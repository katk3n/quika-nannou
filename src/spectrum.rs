use spectrum_analyzer::scaling::*;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

const SAMPLING_RATE: u32 = 44100;

pub fn calc_spectrum(
    samples: &Vec<f32>,
    min_frequency: f32,
    max_frequency: f32,
) -> Vec<(f32, f32)> {
    let hann_window = hann_window(samples.as_slice());
    let spectrum = samples_fft_to_spectrum(
        &hann_window,
        SAMPLING_RATE,
        FrequencyLimit::Range(min_frequency, max_frequency),
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    spectrum
        .data()
        .iter()
        .map(|freq| (freq.0.val(), freq.1.val()))
        .collect()
}
