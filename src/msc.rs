use crate::{Command, FaderPair};
use midir::{MidiOutputConnection, SendError};

pub struct ConsoleMSC {
    midi_conn: MidiOutputConnection,
    device_id: u8,
}

impl ConsoleMSC {
    pub fn new(midi_conn: MidiOutputConnection, device_id: u8) -> ConsoleMSC {
        ConsoleMSC {
            midi_conn,
            device_id,
        }
    }

    pub fn go(&mut self, fader_pair: FaderPair) -> Result<(), SendError> {
        self.send_msc_frame(Command::Go, Some(fader_pair), Some(0))
    }

    pub fn go_cue(&mut self, fader_pair: FaderPair, cue_number: u16) -> Result<(), SendError> {
        self.send_msc_frame(Command::Go, Some(fader_pair), Some(cue_number))
    }

    pub fn stop(&mut self, fader_pair: FaderPair) -> Result<(), SendError> {
        self.send_msc_frame(Command::Stop, Some(fader_pair), Some(0x69))
    }

    pub fn stop_all(&mut self) -> Result<(), SendError> {
        self.send_msc_frame(Command::Stop, None, None)
    }

    pub fn resume(&mut self, fader_pair: FaderPair) -> Result<(), SendError> {
        self.send_msc_frame(Command::Resume, Some(fader_pair), Some(0x69))
    }

    pub fn resume_all(&mut self) -> Result<(), SendError> {
        self.send_msc_frame(Command::Resume, None, None)
    }

    pub fn fire_macro(&mut self, macro_number: u8) -> Result<(), SendError> {
        self.midi_conn.send(&[
            0xF0,
            0x7F,
            self.device_id,
            0x01,
            Command::Fire.value(),
            macro_number,
            0xF7,
        ])
    }

    fn send_msc_frame(
        &mut self,
        command: Command,
        fader_pair: Option<FaderPair>,
        cue_number: Option<u16>,
    ) -> Result<(), SendError> {
        let mut cue_nums: Vec<u8> = vec![];
        let mut fader_pair_bytes: Vec<u8> = vec![];

        if let Some(cue_number) = cue_number {
            cue_nums = cue_number
                .to_string()
                .chars()
                .map(|c| (0x30 + c.to_digit(10).unwrap()) as u8)
                .collect();
            cue_nums.push(0x00);
        }

        if let Some(fader_pair) = fader_pair {
            fader_pair_bytes = vec![fader_pair.value(), 0x00];
        }

        let bytes = [
            vec![0xF0, 0x7F, self.device_id, 0x01, command.value()],
            cue_nums,
            fader_pair_bytes,
            vec![0xF7],
        ]
        .concat();
        self.midi_conn.send(&bytes)
    }
}
