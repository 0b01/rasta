use super::super::SAMPLERATE;
static TUNER_BUFFER_SIZE : usize = SAMPLERATE;

extern crate rustfft;
use effects::{CtrlMsg, Effect};
use self::rustfft::FFTplanner;
use self::rustfft::num_complex::Complex;
use self::rustfft::num_traits::Zero;

pub struct Tuner {
    tuner_buffer: Vec<f32>,
    i_idx: usize,
    bypassing: bool,
}

pub fn calculate_spectrum(samples: &[f32]) -> Vec<f32> {
    let mut input: Vec<Complex<f32>> = samples.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); input.len()];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(input.len());
    fft.process(&mut input, &mut output);

    output.iter()
        .map(|&c| c.norm_sqr())
        .collect()
}

impl Tuner {

    pub fn tune(&self) -> Option<f32> {
        let input = &self.tuner_buffer;

        let input_len = input.len();
        let freqs = calculate_spectrum(input);

        let buckets: Vec<_> =
            (0 .. 1 + input_len / 2) // has Hermitian symmetry to f=0
            .filter_map(|i| {
                let norm = freqs[i];
                let noise_threshold = 1.0;
                if norm > noise_threshold {
                    let f = i as f32 / input_len as f32 * SAMPLERATE as f32;
                    Some((f, norm))
                } else {
                    None
                }
            })
            .collect();

        if buckets.is_empty() {
            return None
        }

        let &(max_f, _max_val) =
            buckets.iter()
            .max_by(|&&(_f1, ref val1), &&(_f2, ref val2)| val1.partial_cmp(val2).unwrap())
            .unwrap();


        println!("Max index is {}", max_f);
        // println!("Max value is {}", max_val);

        Some(max_f)
    }


}

impl Effect for Tuner {

    fn new() -> Self {
        Self {
            tuner_buffer: vec![0.; TUNER_BUFFER_SIZE],
            i_idx: 0,
            bypassing: true,
        }
    }

    fn name(&self) -> &'static str {
        "tuner"
    }

    fn process_samples(&mut self, input: &[f32], _output_l: &mut [f32], _output_r: &mut [f32]) {
        for bufptr in 0..input.len() {
            if self.i_idx >= TUNER_BUFFER_SIZE {
                self.i_idx = 0;
            }
            self.tuner_buffer[self.i_idx] = input[bufptr];
            self.i_idx += 1;
        }
    }

    fn bypass(&mut self) {
        ()
    }

    fn is_bypassing(&self) -> bool {
        self.bypassing
    }

    fn ctrl(&mut self, msg: CtrlMsg) {
        use self::CtrlMsg::*;
        match msg {
            Bypass => self.bypass(),
            _ => (),
        }
    }

}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::{calculate_spectrum, Tuner};
    use effects::Effect;

    #[test]
    fn test_fft() {

        let length = 1024;
        let freq = 2.0;

        let sin_vec: Vec<f32> = (0..length).map(|i| (((i as f32 * freq * 420.0 * PI / length as f32 ).sin() * 0.5))).collect();
        // println!("sin_vec = {:?}", sin_vec);
        let spectrum = calculate_spectrum(sin_vec.as_slice());
        let argmax = {
            let mut argmax = spectrum.iter()
                .enumerate()
                .max_by(
                    |&(_i0, x0), &(_i1, x1)|
                        x0.partial_cmp(x1).unwrap()
                )
                .unwrap()
                .0;

            if argmax > length / 2 {
                argmax = length - argmax;
            }
            argmax
        };

        println!("argmax_spectrum = {}", argmax);
        println!("spectrum = {:?}", spectrum);
    }

    #[test]
    fn test_tune() {
        // okay... this has some "numerical stability" issues surrounding
        // discret fourier transformation when sampling rate >> length of sample
        // there will only be 128 bins if one sample
        // contains 128 data points
        // this means I will have to use a circular buffer
        // to extend the length of the input to fft

        let sampling_rate = 41100.;
        let length = 1280;
        let freq = 420.0;

        let sin_vec: Vec<f32> = (0..length)
            .map(|i| {
                let t = (i as f32) / sampling_rate;
                ((2. * PI) * freq * t).sin() 
            }).collect();

        let tuner = Tuner::new();
        tuner.process_samples(&sin_vec, &mut vec![], &mut vec![]);
        let note = tuner.tune();

        println!("NOTE : {:?}", note);

    }
}
