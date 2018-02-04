//! Takes 2 audio inputs and outputs them to 2 audio outputs.
//! All JACK notifications are also printed out.
 
#![feature(box_syntax)]

extern crate jack;

mod notifications;
mod effects;
mod tuner;

use effects::Effect;
use std::io;
use jack::{Control, Client, ProcessScope};
use notifications::Notifications;
use std::sync::mpsc::channel;

const SAMPLERATE: usize = 48000;
const FRAMES: usize = 128;

fn main() {
    // Create client
    let (client, _status) =
        jack::Client::new("rasta", jack::ClientOptions::NO_START_SERVER).unwrap();

    // Create ports
    let in_b = client
        .register_port("guitar_in", jack::AudioIn::default())
        .unwrap();
    let mut out_a = client
        .register_port("rasta_out_l", jack::AudioOut::default())
        .unwrap();
    let mut out_b = client
        .register_port("rasta_out_r", jack::AudioOut::default())
        .unwrap();

    let mut pedals = effects::EffectProcessor::new();
    // pedals.add(box effects::overdrive::Overdrive::new());
    pedals.add(box effects::delay::Delay::new());

    let (tx, rx) = channel();

    let process_callback = move |_: &Client, ps: &ProcessScope| -> Control {

        let mut out_a_p = out_a.as_mut_slice(ps);
        let mut out_b_p = out_b.as_mut_slice(ps);
        let in_b_p = in_b.as_slice(ps);

        if let Ok(msg) = rx.try_recv() {
            pedals.ctrl(msg);
        }

        // tuner::Tuner::tune(in_b_p, 1./SAMPLERATE as f32);

        pedals.process_samples(in_b_p, &mut out_a_p, &mut out_b_p);

        Control::Continue

    };

    let process = jack::ClosureProcessHandler::new(process_callback);
    let active_client = client.activate_async(Notifications, process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to toggle bypass...");
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        tx.send(effects::CtrlMsg::Bypass).unwrap();
    }

    active_client.deactivate().unwrap();
}
