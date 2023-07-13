use std::{io, io::Write, thread, time};

fn main() {
    let client_1 = etc_express_midi::MidiOutput::new("etc_express_midi Tester").unwrap();
    let ports = client_1.ports();

    println!("\nPorts\n================================");
    for (i, port) in ports.iter().enumerate() {
        println!("{i}: {}", client_1.port_name(port).unwrap());
    }
    println!("================================\n");

    print!("Please enter the index of the target Midi controller: ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let midi_port_index: usize = input_line.trim().parse().expect("Input not an integer");
    println!();

    let midi_conn = client_1
        .connect(&(ports[midi_port_index]), "Testing Output 1")
        .unwrap();
    let midi_channel = 1;
    let mut express_console = etc_express_midi::ConsoleETCMidi::new(midi_conn, midi_channel);

    println!("Sending Go command using ETC Midi");
    express_console.go_cue(etc_express_midi::FaderPair::CD, 1).unwrap();
    println!("Sent first Go command");

    for i in (1..6).rev() {
        println!("{i}");
        thread::sleep(time::Duration::from_secs(1));
    }

    println!("Sending Go command using ETC Midi");
    express_console.go(etc_express_midi::FaderPair::CD).unwrap();
    println!("Sent second Go command");

    for i in (1..6).rev() {
        println!("{i}");
        thread::sleep(time::Duration::from_secs(1));
    }

    println!("Sending Go command using ETC Midi");
    express_console.go(etc_express_midi::FaderPair::CD).unwrap();
    println!("Sent third Go command");
}
