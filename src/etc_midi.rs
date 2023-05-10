use crate::FaderPair;
use midir::{MidiOutputConnection, SendError};

pub struct ConsoleETCMidi<'a> {
    midi_conn: &'a mut MidiOutputConnection,
    midi_chan_num: u8, // Midi channel number 0 to 15
}

impl<'a> ConsoleETCMidi<'a> {
    pub fn new(midi_conn: &'a mut MidiOutputConnection, midi_chan_num: u8) -> ConsoleETCMidi<'a> {
        ConsoleETCMidi {
            midi_conn,
            midi_chan_num,
        }
    }

    pub fn go(&mut self, fader_pair: FaderPair) -> Result<(), SendError> {
        let bytes: Vec<u8> = match fader_pair {
            FaderPair::AB => {
                let byte1 = 0xC0 + self.midi_chan_num; // Byte which contains type of message and Midi channel number
                vec![byte1, 0]
            }
            FaderPair::CD => {
                let byte1 = 0xB0 + self.midi_chan_num; // Byte which contains type of message and Midi channel number
                vec![byte1, 77, 0]
            }
        };

        self.midi_conn.send(bytes.as_slice())
    }

    pub fn go_cue(&mut self, fader_pair: FaderPair, cue_number: u16) -> Result<(), SendError> {
        let bytes: Vec<u8> = match fader_pair {
            FaderPair::AB => {
                let mut message_type = MessageType::ControllerChange;

                let (controller_change, parameter) = match cue_number {
                    0 => {
                        // Error case
                        panic!(
                            "Replace this panic with an error. Also 0 is not a valid cue number"
                        );
                    }
                    1..=127 => {
                        message_type = MessageType::ProgramChange;
                        (77, cue_number)
                    }
                    128..=255 => (78, cue_number - 128),
                    256..=383 => (79, cue_number - 256),
                    384..=511 => (80, cue_number - 384),
                    512..=639 => (81, cue_number - 512),
                    640..=767 => (82, cue_number - 640),
                    768..=895 => (83, cue_number - 768),
                    896..=999 => (84, cue_number - 896),
                    _ => {
                        // Error case: cue number too large
                        panic!("Replace this panic with an error. Also the cue number provided was too large");
                    }
                };

                match message_type {
                    MessageType::ProgramChange => vec![
                        message_type.value() + self.midi_chan_num,
                        parameter.try_into().unwrap(),
                    ],
                    MessageType::ControllerChange => vec![
                        message_type.value() + self.midi_chan_num,
                        controller_change,
                        parameter.try_into().unwrap(),
                    ],
                }
            }
            FaderPair::CD => {
                let byte1 = 0xB0 + self.midi_chan_num; // Byte which contains type of message and Midi channel number

                let (controller_change, parameter) = match cue_number {
                    0 => {
                        // Error case
                        panic!(
                            "Replace this panic with an error. Also 0 is not a valid cue number"
                        );
                    }
                    1..=127 => (77, cue_number),
                    128..=255 => (78, cue_number - 128),
                    256..=383 => (79, cue_number - 256),
                    384..=511 => (80, cue_number - 384),
                    512..=639 => (81, cue_number - 512),
                    640..=767 => (82, cue_number - 640),
                    768..=895 => (83, cue_number - 768),
                    896..=999 => (84, cue_number - 896),
                    _ => {
                        // Error case: cue number too large
                        panic!("Replace this panic with an error. Also the cue number provided was too large");
                    }
                };

                vec![byte1, controller_change, parameter.try_into().unwrap()]
            }
        };

        self.midi_conn.send(bytes.as_slice())
    }
}

enum MessageType {
    ProgramChange,
    ControllerChange,
}

impl MessageType {
    fn value(&self) -> u8 {
        match self {
            MessageType::ProgramChange => 0xC0,
            MessageType::ControllerChange => 0xB0,
        }
    }
}
