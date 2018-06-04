pub mod overdrive;
pub mod delay;
pub mod autowah;
pub mod tuner;
pub mod tremelo;
pub mod pedals;
pub use self::pedals::Pedals;

pub trait Effect: Send {
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

type PedalName = String;
type ConfName = String;
type Val = f32;

pub enum CtrlMsg {
    Bypass,
    BypassPedal(String),
    Tuner,
    Connect(String, String),
    Chain(Vec<CtrlMsg>),
    Disconnect(String),
    Connections,
    Add(String, String),
    Set(PedalName, ConfName, Val),
}
