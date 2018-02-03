use std::slice;

pub mod overdrive;

pub trait Effect : Send {

    fn new() -> Self
        where Self: Sized;

    fn name(&self) -> &'static str;

    fn process_samples(&self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        output_l.clone_from_slice(input);
        output_r.clone_from_slice(input);
    }

    fn bypass(&mut self);

    fn ctrl(&mut self, msg: CtrlMsg);

}

pub struct EffectProcessor {
    pub pedals: Vec<Box<Effect>>,
    pub bypassing: bool,
}

impl EffectProcessor {
    pub fn add(&mut self, eff: Box<Effect>) {
        self.pedals.push(eff)
    }
}

impl Effect for EffectProcessor {

    fn new() -> Self {
        EffectProcessor {
            pedals: vec![],
            bypassing: false,
        }
    }

    fn name(&self) -> &'static str {
        "effects"
    }

    fn process_samples(&self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        let mut is_first = true;
        for eff in self.pedals.iter() {
            if is_first {
                eff.process_samples(input, output_l, output_r);
                is_first = false;
            } else {
                let inp = output_l.to_owned();
                eff.process_samples(&inp, output_l, output_r);
            }
        }
    }

    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
        println!("Bypassing: {}", self.bypassing);
    }

    fn ctrl(&mut self, msg: CtrlMsg) {
        use self::CtrlMsg::*;
        match msg {
            Bypass => self.bypass(),
        }
    }
}

pub enum CtrlMsg {
    Bypass,
}