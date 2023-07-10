use crate::{Error, FaderPair};
use midir::MidiOutputConnection;

/// Control the ETC Express lighting console using ETC's custom
/// MIDI instrument commands.
pub struct ConsoleETCMidi {
    midi_conn: MidiOutputConnection,
    midi_chan_num: u8, // Midi channel number 0 to 15
}

impl ConsoleETCMidi {
    /// Create a new [ConsoleETCMidi]
    ///
    /// Requires an existing [MidiOutputConnection] and the MIDI output channel
    /// number configured in the Express's settings in zero-indexed form (subtract by 1 ex. 12 -> 11).
    ///
    /// ```rust
    /// use etc_express_midi::{MidiOutput, ConsoleETCMidi};
    ///
    /// let midi_client = MidiOutput::new("ETC MIDI Example")?;
    /// let midi_ports = midi_client.ports();
    ///
    /// let midi_port_index = 0; // The index of the desired controller
    /// let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    ///
    /// let express_midi_channel = 0; // The MIDI port from the console offset by -1
    /// let mut express_console = ConsoleETCMidi::new(midi_conn, express_midi_channel);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(midi_conn: MidiOutputConnection, midi_chan_num: u8) -> ConsoleETCMidi {
        ConsoleETCMidi {
            midi_conn,
            midi_chan_num,
        }
    }

    /// Execute the next cue in a [FaderPair]
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleETCMidi, FaderPair};
    /// # let midi_client = MidiOutput::new("ETC MIDI Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let express_midi_channel = 0; // The MIDI port from the console offset by -1
    /// # let mut express_console = ConsoleETCMidi::new(midi_conn, express_midi_channel);
    /// // ...
    ///
    /// express_console.go(FaderPair::AB)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn go(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
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

        self.midi_conn
            .send(bytes.as_slice())
            .map_err(Error::MidiSendError)
    }

    /// Execute a specific cue in a [FaderPair]
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleETCMidi, FaderPair};
    /// # let midi_client = MidiOutput::new("ETC MIDI Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let express_midi_channel = 0; // The MIDI port from the console offset by -1
    /// # let mut express_console = ConsoleETCMidi::new(midi_conn, express_midi_channel);
    /// // ...
    ///
    /// // Seek to cue 1 and execute it in the AB fader pair
    /// express_console.go_cue(FaderPair::AB, 1)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    pub fn go_cue(&mut self, fader_pair: FaderPair, cue_number: u16) -> Result<(), Error> {
        let bytes: Vec<u8> = match fader_pair {
            FaderPair::AB => {
                let mut message_type = MessageType::ControllerChange;

                let (controller_change, parameter) = match cue_number {
                    0 => {
                        return Err(Error::InvalidCue {
                            number: 0,
                            reason: "too small".into(),
                        });
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
                        return Err(Error::InvalidCue {
                            number: cue_number,
                            reason: "too large".into(),
                        });
                    }
                };

                match message_type {
                    MessageType::ProgramChange => vec![
                        message_type.value() + self.midi_chan_num,
                        parameter.try_into()?,
                    ],
                    MessageType::ControllerChange => vec![
                        message_type.value() + self.midi_chan_num,
                        controller_change,
                        parameter.try_into()?,
                    ],
                }
            }
            FaderPair::CD => {
                let byte1 = 0xB0 + self.midi_chan_num; // Byte which contains type of message and Midi channel number

                let (controller_change, parameter) = match cue_number {
                    0 => {
                        return Err(Error::InvalidCue {
                            number: 0,
                            reason: "too small".into(),
                        });
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
                        return Err(Error::InvalidCue {
                            number: cue_number,
                            reason: "too large".into(),
                        });
                    }
                };

                vec![byte1, controller_change, parameter.try_into()?]
            }
        };

        self.midi_conn
            .send(bytes.as_slice())
            .map_err(Error::MidiSendError)
    }
}

/// Represents the MIDI message type to use
enum MessageType {
    ProgramChange,
    ControllerChange,
}

impl MessageType {
    /// Translate the [MessageType] into its binary representation in MIDI
    fn value(&self) -> u8 {
        match self {
            MessageType::ProgramChange => 0xC0,
            MessageType::ControllerChange => 0xB0,
        }
    }
}
