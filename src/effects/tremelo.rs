use effects::{Effect, CtrlMsg};

pub struct Tremelo {
    pub bypassing: bool,
    pub depth: f32,
    pub control: i16,
    pub modulo: i16,
    pub counter_limit: i16,
    pub offset: f32,
    pub alpha_mix: f32,
    pub beta_mix: f32,
}

impl Effect for Tremelo {

    fn new(_sample_rate: usize, _frame_size: u32) -> Self {
        let depth = 1.;
        Tremelo {
            bypassing: false,
            counter_limit: 50,
            depth,
            control: 1,
            modulo: 0,
            offset: 1. - depth,
            alpha_mix: 0.8,
            beta_mix: 0.2,
        }
    }

    fn name(&self) -> &str {
        "tremelo"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        let m = self.modulo as f32 * self.depth / self.counter_limit as f32;
        for (i, x) in input.iter().enumerate() {
            let y_ = (m + self.offset) * x;
            let y = self.mixer(*x, y_);
            output_r[i] = y;
            output_l[i] = y;
        }

        self.sweep();
    }




    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }

    fn is_bypassing(&self) -> bool {
        self.bypassing
    }

    fn ctrl(&mut self, msg: CtrlMsg) {
        use self::CtrlMsg::*;
        match msg {
            Bypass => self.bypass(),
            Set(_pedal_name, conf_name, val) => {
                if &conf_name == "limit" {
                    self.set_limit(val as i16);
                }
                if &conf_name == "mix" {
                    self.set_mixing(val);
                }
            },
            _ => (),
        }

    }

}

impl Tremelo {
    fn sweep(&mut self) {
        self.modulo += self.control;
        if self.modulo > self.counter_limit {
            self.control = -1;
        } else if self.modulo == 0 {
            self.control = 1;
        }
    }
    fn set_limit(&mut self, val: i16) {
        self.counter_limit = val;
    }
    fn mixer(&self, x: f32, y: f32) -> f32 {
        self.alpha_mix * y + self.beta_mix * x
    }
    pub fn set_mixing(&mut self, alpha_mix: f32) {
        self.alpha_mix = alpha_mix;
        self.beta_mix = 1. - alpha_mix;
    }
}
