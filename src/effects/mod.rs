pub mod overdrive;
pub mod delay;
pub mod tuner;

use std::collections::HashMap;

pub trait Effect : Send {

    fn new() -> Self
        where Self: Sized;

    fn name(&self) -> &'static str;

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        output_l.clone_from_slice(input);
        output_r.clone_from_slice(input);
    }

    fn bypass(&mut self);

    fn is_bypassing(&self) -> bool;

    fn ctrl(&mut self, msg: CtrlMsg);

}

pub struct EffectsBox {
    pub pedals: HashMap<&'static str, Box<Effect>>,
    pub bypassing: bool,
    /// in -> eff1 -> eff2 -> out
    chain: HashMap<&'static str, &'static str>,
}

impl Effect for EffectsBox {

    fn new() -> Self {
        EffectsBox {
            pedals: HashMap::new(),
            bypassing: false,
            chain: HashMap::new(),
        }
    }

    fn name(&self) -> &'static str {
        "effects"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {

        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        let mut is_first = true;
        let mut next_node = self.chain.get("in").unwrap();

        while next_node != &"out" {
            let eff = self.pedals.get_mut(next_node).unwrap();
            if eff.is_bypassing() {
                continue;
            }
            if is_first {
                eff.process_samples(input, output_l, output_r);
                is_first = false;
            } else {
                let inp = output_l.to_owned();
                eff.process_samples(&inp, output_l, output_r);
            }

            next_node = self.chain.get(next_node).unwrap();
        }

    }

    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
        println!("Bypassing: {}", self.bypassing);
    }

    fn is_bypassing(&self) -> bool {
        self.bypassing
    }

    fn ctrl(&mut self, msg: CtrlMsg) {
        use self::CtrlMsg::*;
        match msg {
            Bypass => self.bypass(),
            Tuner => (*self.pedals.get_mut("tuner").unwrap()).ctrl(msg)
        }
    }

}

impl EffectsBox {

    pub fn add(&mut self, name: &'static str, eff: Box<Effect>) {
        self.pedals.insert(name, eff);
    }

    pub fn connect(&mut self, from: &'static str, to: &'static str) {
        self.chain.insert(from, to).unwrap();
    }

}

pub enum CtrlMsg {
    Bypass,
    Tuner,
}