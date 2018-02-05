use effects::{CtrlMsg, Effect};
use std::default::Default;
use std::f32::consts::PI as pi;

const sinConst3: f32 = -1. / 6.;
const sinConst5: f32 = 1. / 120.;
const tanConst3: f32 = 1. / 3.;
const tanConst5: f32 = 1. / 3.;

fn sin(x: f32) -> f32 {
    x * (1. + sinConst3 * x * x)
}

fn precisionSin(x: f32) -> f32 {
    let x2 = x * x;
    let x4 = x2 * x2;
    x * (1. + sinConst3*x2 + sinConst5*x4)
}
fn tan(x: f32) -> f32 {
    x * (1. + tanConst3*x*x)
}

fn precisionTan(x: f32) -> f32 {
    let x2 = x * x;
    let x4 = x2 * x2;
    x * (1. + tanConst3*x2 + tanConst5*x4)
}

#[derive(Default)]
pub struct AutoWah {
    bypassing: bool,
    frame_size: u32,

    // Level Detector parameters
    alphaA: f32,
    alphaR: f32,
    betaA: f32,
    betaR: f32,
    bufferL: (f32, f32),

    // Lowpass filter parameters
    bufferLP: f32,
    gainLP: f32,

    // State Variable Filter parameters
    minFreq: f32,
    freqBandwidth: f32,
    q: f32,
    sample_rate: f32,
    centerFreq: f32,
    yHighpass: f32,
    yBandpass: f32,
    yLowpass: f32,
    filter: FilterType,

    // Mixer parameters
    alphaMix: f32,
    betaMix: f32,
}

impl Effect for AutoWah {
    fn new(sample_rate: usize, frame_size: u32) -> Self {
        let mut aw = AutoWah {
            bypassing: false,
            sample_rate: sample_rate as f32,
            frame_size,
            ..Default::default()
        };

        aw.set_attack(0.04);
        aw.set_release(0.002);
        aw.set_min_maxFreq(20., 3000.);
        aw.set_quality_factor(1. / 5.);
        aw.set_mixing(0.8);

        aw
    }

    fn name(&self) -> &str {
        "autowah"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        for i in 0..self.frame_size as usize {
            let x = input[i] * 1.;
            let mut y = self.run_effect(x) * 2.;

            //TODO: saturation
            if y > 1. {y = 1.;}
            else if y < -1. {y = -1.;}

            output_l[i] = y;
            output_r[i] = y;
        }
    }

    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }

    fn is_bypassing(&self) -> bool {
        self.bypassing
    }

    fn ctrl(&mut self, msg: CtrlMsg) {
        match msg {
            _ => (),
        }
    }
}

impl AutoWah {
    pub fn run_effect(&mut self, x: f32) -> f32 {
        let xL = x.abs();

        let yL = self.level_detector(xL);

        //fc = yL * (maxFreq - minFreq) + minFreq;
        self.centerFreq = yL * self.freqBandwidth + self.minFreq;

        //float xF = x;
        let xF = self.low_pass_filter(x);
        let yF = self.state_variable_filter(xF);

        let y = self.mixer(x, yF);

        return y;
    }

    pub fn set_filter_type(&mut self, typ: FilterType) {
        self.filter = typ;
    }
    pub fn set_attack(&mut self, tauA: f32) {
        self.alphaA = (-1. / tauA / self.sample_rate ).exp();
        self.betaA = 1. - self.alphaA;
    }
    pub fn set_release(&mut self, tauR: f32) {
        self.alphaR = (-1. / tauR / self.sample_rate ).exp();
        self.betaR = 1. - self.alphaA;
    }
    pub fn set_min_maxFreq(&mut self, minFreq: f32, maxFreq: f32) {
        self.freqBandwidth = pi * (2. * maxFreq - minFreq) / self.sample_rate;
        self.minFreq = pi * minFreq / self.sample_rate;
    }
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
    pub fn set_quality_factor(&mut self, Q: f32) {
        self.q = Q;
        self.gainLP = (0.5 * Q).sqrt();
    }
    pub fn set_mixing(&mut self, alphaMix: f32) {
        self.alphaMix = alphaMix;
        self.betaMix = 1. - alphaMix;
    }
    fn level_detector(&mut self, x: f32) -> f32 {
        let y1 = self.alphaR * self.bufferL.1 + self.betaR * x;
        if x > y1 { self.bufferL.1 = x; }
        else      { self.bufferL.1 = y1;}

        self.bufferL.0 = self.alphaA * self.bufferL.0 + self.betaA * self.bufferL.1;

        return self.bufferL.0;
    }
    fn low_pass_filter(&mut self, x: f32) -> f32 {
        let K = tan(self.centerFreq);
        let b0 = K / (K + 1.);
        let a1 = 2.0 * (b0 - 0.5);

        let xh = x - a1 * self.bufferLP;
        let y = b0 * (xh + self.bufferLP);
        self.bufferLP = xh;

        return self.gainLP * y;
    }
    fn state_variable_filter(&mut self, x: f32) -> f32{
        let f = 2.0 * sin(self.centerFreq);
        self.yHighpass  = x - self.yLowpass - self.q * self.yBandpass;
        self.yBandpass += f * self.yHighpass;
        self.yLowpass  += f * self.yBandpass;

        use self::FilterType::*;
        match self.filter {
            Lowpass => self.yLowpass,
            Bandpass => self.yBandpass,
            Highpass => self.yHighpass,
        }
    }
    fn mixer(&self, x: f32, y: f32) -> f32 {
        self.alphaMix * y + self.betaMix * x
    }
}

enum FilterType {
    Lowpass,
    Bandpass,
    Highpass
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::Highpass
    }
}