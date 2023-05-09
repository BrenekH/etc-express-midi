pub use midir::MidiOutput;
use midir::{MidiOutputConnection, SendError};

pub enum FaderPair {
    AB,
    CD,
}

impl FaderPair {
    fn value(&self) -> u8 {
        match self {
            FaderPair::AB => 49,
            FaderPair::CD => 50,
        }
    }
}

enum Command {
    Go = 1,
    Stop = 2,
    Resume = 3,
    Fire = 4,
}

impl Command {
    fn value(&self) -> u8 {
        match self {
            Command::Go => 1,
            Command::Stop => 2,
            Command::Resume => 3,
            Command::Fire => 4,
        }
    }
}

pub fn go_msc(
    conn: &mut MidiOutputConnection,
    device_id: u8,
    fader_pair: FaderPair,
) -> Result<(), SendError> {
    // MSC Go Command: F0 7F <device_id> 01(type) 01 30 00 <fader_pair> 00 F7
    // <Sysex start(F0 7F)> <device_id> <msg type> <cmd> <cue 0> <delimiter> <fader pair> <delimiter> <sysex end>

    conn.send(&[
        0xF0,
        0x7F,
        device_id,
        0x01,
        Command::Go.value(),
        0x30,
        0x00,
        fader_pair.value(),
        0x00,
        0xF7,
    ])
}

pub fn go_etc_midi(
    conn: &mut MidiOutputConnection,
    midi_chan_num: u8, // Midi channel number 0 to 15
    fader_pair: FaderPair,
) -> Result<(), SendError> {
    let bytes: Vec<u8> = match fader_pair {
        FaderPair::AB => {
            let byte1 = 0xC0 + midi_chan_num; // Byte which contains type of message and Midi channel number
            vec![byte1, 0]
        }
        FaderPair::CD => {
            let byte1 = 0xB0 + midi_chan_num; // Byte which contains type of message and Midi channel number
            vec![byte1, 77, 0]
        }
    };

    conn.send(bytes.as_slice())
}
