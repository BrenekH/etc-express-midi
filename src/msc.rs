use crate::{Command, FaderPair};
use midir::{MidiOutputConnection, SendError};

pub struct ConsoleMSC<'a> {
    midi_conn: &'a mut MidiOutputConnection,
    device_id: u8,
}

impl<'a> ConsoleMSC<'a> {
    pub fn new(midi_conn: &'a mut MidiOutputConnection, device_id: u8) -> ConsoleMSC<'a> {
        ConsoleMSC {
            midi_conn,
            device_id,
        }
    }

    pub fn go(&mut self, fader_pair: FaderPair) -> Result<(), SendError> {
        self.send_msc_frame(Command::Go, 0, fader_pair.value())
    }

    pub fn go_cue() {}

    pub fn stop() {}

    pub fn stop_all() {}

    pub fn resume() {}

    pub fn resume_all() {}

    fn send_msc_frame(&mut self, command: Command, cue_number: u8, list_entry: u8) -> Result<(), SendError> {
        // TODO: Serialize cue_number into a sequence of bytes that the Express will accept
        let cue_nums: Vec<u8> = cue_number.to_string().chars().map(|c| (0x30 + c.to_digit(10).unwrap()) as u8).collect();

        let bytes = [vec![0xF0, 0x7F, self.device_id, 0x01, command.value()], cue_nums, vec![0x00, list_entry, 0x00, 0xF7]].concat();
        self.midi_conn.send(&bytes)
    }
}
