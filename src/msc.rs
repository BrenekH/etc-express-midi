use crate::{Error, FaderPair};
use midir::MidiOutputConnection;

/// Control the ETC Express lighting console using MIDI Show Control
/// (requires a controller which passes SysEx messages).
pub struct ConsoleMSC {
    midi_conn: MidiOutputConnection,
    device_id: u8,
}

impl ConsoleMSC {
    /// Create a new [ConsoleMSC]
    ///
    /// ```
    /// use etc_express_midi::{MidiOutput, ConsoleMSC};
    ///
    /// let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// let midi_ports = midi_client.ports();
    ///
    /// let midi_port_index = 0; // The index of the desired controller
    /// let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    ///
    /// let msc_device_id = 1;
    /// let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(midi_conn: MidiOutputConnection, device_id: u8) -> ConsoleMSC {
        ConsoleMSC {
            midi_conn,
            device_id,
        }
    }

    /// Execute the next cue in a [FaderPair]
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC, FaderPair};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// express_console.go(FaderPair::AB)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn go(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
        self.send_msc_frame(Command::Go, Some(fader_pair), Some(0))
    }

    /// Execute a specific cue in a [FaderPair]
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC, FaderPair};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// // Seek to cue 1 and execute it in the AB fader pair
    /// express_console.go_cue(FaderPair::AB, 1)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn go_cue(&mut self, fader_pair: FaderPair, cue_number: u16) -> Result<(), Error> {
        self.send_msc_frame(Command::Go, Some(fader_pair), Some(cue_number))
    }

    /// Stop the currently executing cue in a specific [FaderPair]
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC, FaderPair};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// express_console.stop(FaderPair::AB)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn stop(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
        self.send_msc_frame(Command::Stop, Some(fader_pair), Some(0x69))
    }

    /// Stop all currently executing cues
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// express_console.stop_all()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn stop_all(&mut self) -> Result<(), Error> {
        self.send_msc_frame(Command::Stop, None, None)
    }

    /// Resume the cue in a specific [FaderPair]
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC, FaderPair};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// express_console.resume(FaderPair::AB)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn resume(&mut self, fader_pair: FaderPair) -> Result<(), Error> {
        self.send_msc_frame(Command::Resume, Some(fader_pair), Some(0x69))
    }

    /// Resume all cues currently stopped
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// express_console.resume_all()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn resume_all(&mut self) -> Result<(), Error> {
        self.send_msc_frame(Command::Resume, None, None)
    }

    /// Run a specific macro
    ///
    /// ```rust
    /// # use etc_express_midi::{MidiOutput, ConsoleMSC};
    /// # let midi_client = MidiOutput::new("ETC MSC Example")?;
    /// # let midi_ports = midi_client.ports();
    /// # let midi_port_index = 0; // The index of the desired controller
    /// # let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;
    /// # let msc_device_id = 0;
    /// # let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);
    /// // ...
    ///
    /// express_console.fire_macro(10)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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

    /// Sends an MSC frame constructed from the provided parameters
    /// to the [MidiOutputConnection] stored in the struct.
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
