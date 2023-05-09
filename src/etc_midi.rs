use crate::FaderPair;
use midir::{MidiOutputConnection, SendError};

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
