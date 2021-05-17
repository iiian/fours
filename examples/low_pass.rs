use fours::{self, SignalGenerator};
use num_complex::Complex32;
use plotters;

fn main() {
    let bins = vec![
        0, 1000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]
    .into_iter()
    .map(|e| Complex32::new(e as f32, 0.0))
    .collect::<Vec<_>>();
    let sig_one = SignalGenerator::new(&bins[..])
        .into_iter()
        .map(|e| e.norm());

    let bins2 = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 300, 300, 300, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ]
    .into_iter()
    .map(|e| Complex32::new(e as f32, 0.0))
    .collect::<Vec<_>>();
    let sig_two = SignalGenerator::new(&bins2[..])
        .into_iter()
        .map(|e| e.norm());

    let composite_signal = sig_one.cloned().into();
}
