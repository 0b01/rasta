pub mod overdrive;
pub mod delay;
pub mod tuner;

use std::collections::HashMap;

pub trait Effect : Send {

    fn new(sample_rate: usize, frame_size: u32) -> Self
        where Self: Sized;

    fn name(&self) -> &str;

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        output_l.clone_from_slice(input);
        output_r.clone_from_slice(input);
    }

    fn bypass(&mut self);

    fn is_bypassing(&self) -> bool;

    fn ctrl(&mut self, msg: CtrlMsg);

}

pub struct EffectsBox {
    sample_rate: usize,
    frame_size: u32,
    pub pedals: HashMap<String, Box<Effect>>,
    pub bypassing: bool,
    /// in -> eff1 -> eff2 -> out
    chain: HashMap<String, String>,
}

impl Effect for EffectsBox {

    fn new(sample_rate: usize, frame_size: u32) -> Self {
        EffectsBox {
            sample_rate,
            frame_size,
            pedals: HashMap::new(),
            bypassing: false,
            chain: HashMap::new(),
        }
    }

    fn name(&self) -> &str {
        "effects"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {

        if self.bypassing {
            output_l.clone_from_slice(input);
            output_r.clone_from_slice(input);
            return;
        }

        let mut next_node = self.chain.get("in");
        let mut next_node = if next_node.is_none() {
            return;
        } else {
            next_node.unwrap()
        };

        let mut temp_buf = input.to_owned();

        while *next_node != "out" {
            let eff = self.pedals.get_mut(next_node);
            let eff = if eff.is_none() {
                break
            } else {
                eff.unwrap()
            };
            // if eff.is_bypassing() { continue; }

            eff.process_samples(&temp_buf, output_l, output_r);
            temp_buf = output_l.to_owned();

            let next = self.chain.get(next_node);

            next_node = if next.is_none() {
                break
            } else {
                next.unwrap()
            };
        }

        output_l.clone_from_slice(&temp_buf);
        output_r.clone_from_slice(&temp_buf);

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
            BypassPedal(name) => {
                let mut pedal = self.pedals.get_mut(&name).unwrap();
                (*pedal).ctrl(Bypass);
            }
            Tuner => {
                let mut tuner = self.pedals.get_mut("tuner").unwrap();
                (*tuner).ctrl(msg);
            },
            Connect(from, to) => {
                self.connect(&from, &to)
            },
            Disconnect(from) => {
                self.disconnect(&from)
            },
            Connections => self.print_conn(),
            Add(name, eff_type) => {
                let eff : Box<Effect> = match eff_type.as_str() {
                    "delay" => box delay::Delay::new(self.sample_rate, self.frame_size),
                    "overdrive" => box overdrive::Overdrive::new(self.sample_rate, self.frame_size),
                    &_ => unimplemented!()
                };
                self.add(&name, eff);
            },
            Set(name, conf, val) => {
                let mut pedal = self.pedals.get_mut(&name).unwrap();
                (*pedal).ctrl(Set(name, conf, val));
            },
            Chain(v) => {
                for i in v.into_iter() {
                    self.ctrl(i);
                }
            }
        }
    }

}

impl EffectsBox {

    pub fn add(&mut self, name: &str, eff: Box<Effect>) {
        self.pedals.insert(name.to_owned(), eff);
    }

    pub fn connect(&mut self, from: &str, to: &str) {
        self.chain.insert(from.to_owned(), to.to_owned());
    }

    pub fn disconnect(&mut self, from: &str) {
        self.chain.remove(from);
    }

    pub fn print_conn(&self) {

        print!("Chain: ");
        let mut node = "in";
        while node != "out" && self.chain.contains_key(node) {
            print!("{} -> ", node);
            node = self.chain.get(node).unwrap();
        }
        println!("out");

        println!("Graph: {:?}", self.chain);

        println!("Pedals: {:?}", self.pedals.keys().collect::<Vec<_>>())
    }

}

pub enum CtrlMsg {
    Bypass,
    BypassPedal(String),
    Tuner,
    Connect(String, String), 
    Chain(Vec<CtrlMsg>),
    Disconnect(String),
    Connections,
    Add(String, String),
    Set(String/*pedal*/, String/*confname*/, f32/*val*/),
}