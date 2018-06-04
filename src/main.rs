//! Takes 2 audio inputs and outputs them to 2 audio outputs.
//! All JACK notifications are also printed out.

#![feature(box_syntax)]

extern crate jack;

mod notifications;
mod effects;
mod parser;

use parser::parse_input;
use effects::{Effect};
use std::io::{self, Write};
use jack::{Control, Client, ProcessScope};
use std::sync::mpsc::channel;

fn main() {
    // Create client
    let (client, _status) =
        jack::Client::new("rasta", jack::ClientOptions::NO_START_SERVER).unwrap();

    let sample_rate = client.sample_rate();
    let frame_size = client.buffer_size();

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

    let mut pedals = effects::Pedals::new(sample_rate, frame_size);
    pedals.add("overdrive", box effects::overdrive::Overdrive::new(sample_rate, frame_size));
    pedals.add("delay", box effects::delay::Delay::new(sample_rate, frame_size));
    pedals.add("tuner", box effects::tuner::Tuner::new(sample_rate, frame_size));
    pedals.add("aw", box effects::autowah::AutoWah::new(sample_rate, frame_size));
    pedals.add("trem", box effects::tremelo::Tremelo::new(sample_rate, frame_size));

    let (tx, rx) = channel();

    let process_callback = move |_: &Client, ps: &ProcessScope| -> Control {

        let mut out_a_p = out_a.as_mut_slice(ps);
        let mut out_b_p = out_b.as_mut_slice(ps);
        let in_b_p = in_b.as_slice(ps);

        if let Ok(msg) = rx.try_recv() {
            pedals.ctrl(msg);
        }
        pedals.process_samples(in_b_p, &mut out_a_p, &mut out_b_p);
        Control::Continue
    };

    let process = jack::ClosureProcessHandler::new(process_callback);
    let active_client = client.activate_async((), process).unwrap();

    // Wait for user input to quit
    let mut user_input = String::new();
    print!(">>> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        let msg = parse_input(&user_input[0..user_input.len()-1]);
        tx.send(msg).unwrap();
        user_input.clear();
        print!(">>> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
    }

    active_client.deactivate().unwrap();
}
