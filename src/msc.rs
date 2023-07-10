use crate::{Error, FaderPair};
use midir::MidiOutputConnection;

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

    pub fn go(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
        self.send_msc_frame(Command::Go, Some(fader_pair), Some(0))
    }

    pub fn go_cue(&mut self, fader_pair: FaderPair, cue_number: u16) -> Result<(), Error> {
        self.send_msc_frame(Command::Go, Some(fader_pair), Some(cue_number))
    }

    pub fn stop(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
        self.send_msc_frame(Command::Stop, Some(fader_pair), Some(0x69))
    }

    pub fn stop_all(&mut self) -> Result<(), Error> {
        self.send_msc_frame(Command::Stop, None, None)
    }

    pub fn resume(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
        self.send_msc_frame(Command::Resume, Some(fader_pair), Some(0x69))
    }

    pub fn resume_all(&mut self) -> Result<(), Error> {
        self.send_msc_frame(Command::Resume, None, None)
    }

    pub fn fire_macro(&mut self, macro_number: u8) -> Result<(), Error> {
        self.midi_conn
            .send(&[
                0xF0,
                0x7F,
                self.device_id,
                0x01,
                Command::Fire.value(),
                macro_number,
                0xF7,
            ])
            .map_err(Error::MidiSendError)
    }

    fn send_msc_frame(
        &mut self,
        command: Command,
        fader_pair: Option<FaderPair>,
        cue_number: Option<u16>,
    ) -> Result<(), Error> {
        let mut cue_nums: Vec<u8> = vec![];
        let mut fader_pair_bytes: Vec<u8> = vec![];

        if let Some(cue_number) = cue_number {
            cue_nums = cue_number
                .to_string()
                .chars()
                // The following unwrap is acceptable because it will only panic if the character
                // is invalid for base 10, which it will always be because we are converting from
                // base 10.
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
        self.midi_conn
            .send(&bytes)
            .map_err(Error::MidiSendError)
    }
}

/// The possible commands that can be sent to the console
enum Command {
    Go = 1,
    Stop = 2,
    Resume = 3,
    Fire = 4,
}

impl Command {
    /// Maps a [Command] to a binary value that can be used in MIDI commands
    fn value(&self) -> u8 {
        match self {
            Command::Go => 1,
            Command::Stop => 2,
            Command::Resume => 3,
            Command::Fire => 4,
        }
    }
}
