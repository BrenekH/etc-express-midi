mod etc_midi;
mod msc;

// Library exports
pub use etc_midi::ConsoleETCMidi;
pub use msc::ConsoleMSC;

// Re-exports
pub use midir::MidiOutput;

pub enum FaderPair {
    AB,
    CD,
}

impl FaderPair {
    fn value(&self) -> u8 {
        match self {
            FaderPair::AB => 0x31,
            FaderPair::CD => 0x32,
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
