use effects::{Effect, CtrlMsg};

pub struct Overdrive {
    pub bypassing: bool,
}

/// Audio at a low input level is driven by higher input
/// levels in a non-linear curve characteristic
/// 
/// For overdrive, Symmetrical soft clipping of input values has to
/// be performed.
impl Effect for Overdrive {

    fn new(_sample_rate: usize, _frame_size: u32) -> Self {
        Overdrive {
            bypassing: false
        }
    }

    fn name(&self) -> &str {
        "overdrive"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {

        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        let slice = input.iter().map(|&x| {
            let x = x.abs();
            if 0. < x  && x < 0.333 {
                2. * x
            } else if 0.333 < x && x < 0.666 {
                let t = 2. - 3. * x;
                (3. - t * t) / 3.
            } else {
                x
            }
        }).collect::<Vec<f32>>();

        output_l.clone_from_slice(&slice);
        output_r.clone_from_slice(&slice);
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
            _ => (),
        }

    }

}