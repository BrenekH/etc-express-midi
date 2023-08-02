# ETC Express MIDI

Rust library to control the [ETC Express](https://www.etcconnect.com/Products/Legacy/Console/Others/Express/Support-Articles.aspx) lighting console using MIDI commands.

This library contains 2 methods of interacting with the Express console:
- [Method 1](ConsoleETCMidi) uses MIDI instrument commands to interact with the cue and macro functionality
  of the Express.
  Any USB to MIDI adapter should work with this method.

- [Method 2](ConsoleMSC) uses MIDI Show Control to operate the Express.
  This method requires a MIDI adapter that passes [SysEx](https://en.wikipedia.org/wiki/MIDI#System_Exclusive_messages) commands.

## Examples

### ETC MIDI

```rust
use etc_express_midi::{MidiOutput, ConsoleETCMidi, FaderPair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let midi_client = MidiOutput::new("ETC MIDI Example")?;
    let midi_ports = midi_client.ports();

    let midi_port_index = 0; // The index of the desired controller
    let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;

    let express_midi_channel = 1;
    let mut express_console = ConsoleETCMidi::new(midi_conn, express_midi_channel);

    // Execute the next cue in the CD fader pair
    express_console.go(FaderPair::CD)?;

    Ok(())
}
```

### MIDI Show Control

```rust
use etc_express_midi::{MidiOutput, ConsoleMSC, FaderPair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let midi_client = MidiOutput::new("ETC MSC Example")?;
    let midi_ports = midi_client.ports();

    let midi_port_index = 0; // The index of the desired controller
    let midi_conn = midi_client.connect(&(midi_ports[midi_port_index]), "Example Output")?;

    let msc_device_id = 1;
    let mut express_console = ConsoleMSC::new(midi_conn, msc_device_id);

    // Execute the next cue in the CD fader pair
    express_console.go(FaderPair::CD)?;

    Ok(())
}
```
