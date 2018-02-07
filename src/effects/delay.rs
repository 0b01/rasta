use effects::{Effect, CtrlMsg};

pub struct Delay {
    pub bypassing: bool,
    delay_buffer: Vec<f32>,
    delay_buffer_size: usize,
    feedback: f32,
    i_idx: usize,
    o_idx: usize,
    sample_rate: usize,
    delay_time: usize,
    frame_size: u32,
}

impl Delay {

    /// t is in seconds
    pub fn set_delay(&mut self, t: f32) {
        let delay_time = (t * self.sample_rate as f32) as usize;
        assert!(delay_time < self.delay_buffer_size);
        self.delay_time = delay_time;
    }

    pub fn set_feedback(&mut self, f: f32) {
        assert!(f < 1.); // multiplying by > 1 would be too loud
        self.feedback = f;
    }


}

impl Effect for Delay {

    fn new(sample_rate: usize, frame_size: u32) -> Self {
        let dbs = sample_rate;
        Delay {
            bypassing: false,
            delay_buffer_size: dbs,
            delay_buffer: vec![0.; dbs],
            feedback: 0.3,
            i_idx: 0,
            o_idx: 0,
            delay_time: 8820,
            sample_rate,
            frame_size
        }
    }

    fn name(&self) -> &str {
        "delay"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {

        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        for bufptr in 0..self.frame_size as usize {
            if self.i_idx >= self.delay_buffer_size {
                self.i_idx = 0;
            }
            
            self.o_idx = if self.i_idx >= self.delay_time {
                self.i_idx - self.delay_time
            } else {
                self.delay_buffer_size as usize + self.i_idx - self.delay_time
            };

            let y = input[bufptr] + (self.delay_buffer[self.o_idx] * self.feedback);
            self.delay_buffer[self.i_idx] = y;
            let out = (y + 0.5).cos();

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
            Set(_pedal_name, conf_name, val) => {
                if &conf_name == "feedback" {
                    self.set_feedback(val);
                } else if &conf_name == "delay" {
                    self.set_delay(val);
                }
            },
            _ => (),
        }
    }

}