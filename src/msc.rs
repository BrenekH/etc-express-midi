use crate::{Command, FaderPair};
use midir::{MidiOutputConnection, SendError};

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
