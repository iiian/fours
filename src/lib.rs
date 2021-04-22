use num_complex::Complex32;

/// An implementation of the true discrete fourier transform.
///
///
///
/// This function will return a
/// [Vec] where each element is defined by the following formula:
/// ```text
///          N - 1
///          ----/
///          \             j2πkn/N
/// X[n]  =   >    x[n] * e
///          /  
///          ----\
///          n = 0
/// ```
/// where:
/// - `x` is the input time series,
/// - `n` is an index into the time series (for sampling)
/// - `k` is an integer s.t. `0 <= k < N`
/// - `N` is the length of the time series in samples
/// - `X[n]` are the output frequency "bins"
///
/// Note that the result is in the complex domain. The standard way to convert out of the complex
/// domain after a fourier transform is to take the magnitude of each bin:
/// ```rust
/// use fours::dft;
/// let sample_data: &[i32] = &vec![-50, 0, 50, 0, -50, 0, 50, 0][..];
/// let complex_bins = dft(sample_data);
/// let bins: Vec<_> = complex_bins.iter().map(|b| b.norm()).collect();
/// ```
pub fn dft(time_series: &[i32]) -> Vec<Complex32> {
    use std::f32::consts::PI;
    let l = time_series.len();
    let mut x_h = vec![];
    for k in 0..l as u32 {
        x_h.push(
            time_series
                .iter()
                .enumerate()
                .map(|(n, x_n)| {
                    let arg = Complex32::new(0.0, -2.0 * PI * k as f32 * n as f32 / l as f32);
                    Complex32::from(*x_n as f32) * arg.exp()
                })
                .sum(),
        );
    }
    x_h
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_should_work() {
        let mut bit = true;
        let time_series: Vec<_> = (0..10)
            .map(|_| {
                let bit_was = bit;
                bit = !bit;
                if bit_was {
                    i32::max_value()
                } else {
                    i32::min_value()
                }
            })
            .collect();

        let freq_analysis = dft(&time_series[..]);
        let max: f32 = freq_analysis
            .iter()
            .map(|e| e.norm())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        for (index, bin) in freq_analysis.iter().enumerate() {
            let value = bin.norm() / max;
            if index == 5 {
                assert!(value > 0.9999999, "Frequency bin {} was {}", index, value);
                continue;
            }
            assert!(value < 0.00001, "Frequency bin {} was {}", index, value);
        }
    }
}
