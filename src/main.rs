use std::{io, io::Write, thread, time};

fn main() {
    let client_1 = etc_express_midi::MidiOutput::new("Client 1").unwrap();
    let ports = client_1.ports();

    println!("\nPorts\n================================");
    for (i, port) in ports.iter().enumerate() {
        println!("{i}: {}", client_1.port_name(&port).unwrap());
    }
    println!("================================\n");

    print!("Please enter the index of the target Midi controller: ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let midi_port_index: usize = input_line.trim().parse().expect("Input not an integer");
    println!("");

    let mut midi_conn = client_1.connect(&(ports[midi_port_index]), "Test").unwrap();
    let device_id = 16;
    println!("Sending Go command using Midi Show Control");
    etc_express_midi::go_msc(&mut midi_conn, device_id, etc_express_midi::FaderPair::CD).unwrap();
    println!("Sent first Go command");

    for i in (1..6).rev() {
        println!("{i}");
        thread::sleep(time::Duration::from_secs(1));
    }

    let midi_channel = 2;
    println!("Sending Go command using ETC Midi");
    etc_express_midi::go_etc_midi(
        &mut midi_conn,
        midi_channel,
        etc_express_midi::FaderPair::CD,
    )
    .unwrap();
    println!("Sent second Go command");
}
