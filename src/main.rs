use std::{thread, time};

fn main() {
    let client_1 = etc_express_midi::MidiOutput::new("Client 1").unwrap();
    let ports = client_1.ports();

    println!("\nPorts\n================================");
    for (i, port) in ports.iter().enumerate() {
        println!("{i}: {}", client_1.port_name(&port).unwrap());
    }
    println!("================================\n");

    let mut midi_conn = client_1.connect(&(ports[0]), "Test").unwrap();
    let device_id = 16;
    println!("Sending Go command using Midi Show Control");
    etc_express_midi::go_msc(&mut midi_conn, device_id, etc_express_midi::FaderPair::AB).unwrap();
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
        etc_express_midi::FaderPair::AB,
    )
    .unwrap();
    println!("Sent second Go command");
}
