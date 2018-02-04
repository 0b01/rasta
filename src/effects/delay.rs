use effects::{Effect, CtrlMsg};
use super::super::{SAMPLERATE, FRAMES};

static DELAY_BUFFER_SIZE : usize = SAMPLERATE;

pub struct Delay {
    pub bypassing: bool,
    delay_buffer: Vec<f32>,
    feedback: f32,
    i_idx: usize,
    o_idx: usize,
    delay_time: usize,
}

impl Delay {

    pub fn set_delay(&mut self, t: usize) {
        self.delay_time = t;
    }

    pub fn set_feedback(&mut self, f: f32) {
        self.feedback = f;
    }


}

impl Effect for Delay {

    fn new() -> Self {
        Delay {
            bypassing: false,
            delay_buffer: vec![0.; DELAY_BUFFER_SIZE],
            feedback: 0.5,
            i_idx: 0,
            o_idx: 0,
            delay_time: 5000,
        }
    }

    fn name(&self) -> &'static str {
        "delay"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {

        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        for bufptr in 0..FRAMES {
            if self.i_idx >= DELAY_BUFFER_SIZE {
                self.i_idx = 0;
            }
            
            self.o_idx = if self.i_idx >= self.delay_time {
                self.i_idx - self.delay_time
            } else {
                DELAY_BUFFER_SIZE + self.i_idx - self.delay_time
            };
            
            self.delay_buffer[self.i_idx] = input[bufptr] + (self.delay_buffer[self.o_idx] * self.feedback);
            let out = (self.delay_buffer[self.i_idx] + 0.5).cos();

            output_r[bufptr] = out;
            output_l[bufptr] = out;
            
            self.i_idx += 1;
        }

        // output_l.clone_from_slice(&slice);
        // output_r.clone_from_slice(&slice);
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