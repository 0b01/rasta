use effects::{Effect, CtrlMsg};

pub struct Tremelo {
    pub bypassing: bool,
    pub depth: f32,
    pub control: i16,
    pub modulo: i16,
    pub counter_limit: i16,
    pub offset: f32,
}

impl Effect for Tremelo {

    fn new(_sample_rate: usize, _frame_size: u32) -> Self {
        let depth = 1.;
        Tremelo {
            bypassing: false,
            counter_limit: 4000,
            depth,
            control: 1,
            modulo: 0,
            offset: 1. - depth,
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
            let y = (m + self.offset) * x;
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
        assert!(val >= 100);
        self.counter_limit = val;
    }
}
