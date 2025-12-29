use anyhow::{Context, Result, anyhow, bail};
use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort};

use crate::backend::{timecode::Timecode, timecode_decoder::TimecodeDecoder};

pub struct MtcTimecodeDecoder {
    pub fps: u8,
    pub port: Option<MidiInputPort>,
    current_timecode: Timecode,
    connection: Option<MidiInputConnection<()>>,
    midi_in: MidiInput,
}

impl MtcTimecodeDecoder {
    pub fn new() -> Result<Self> {
        let mut midi_in = MidiInput::new("technus timecode reading input check ports")?;
        midi_in.ignore(Ignore::None);

        Ok(Self {
            fps: 0,
            port: None,
            current_timecode: Timecode::default(),
            connection: None,
            midi_in,
        })
    }

    /// Gets the currently active ports on the system. This is an I/O operation so don't spam it
    pub fn get_ports(&self) -> Vec<MidiInputPort> {
        self.midi_in.ports()
    }

    pub fn port_name(&self, port: &MidiInputPort) -> Result<String> {
        Ok(self.midi_in.port_name(port)?)
    }

    /// Connect to the specified MIDI port and start updating timecode.
    fn connect(&mut self) -> Result<()> {
        let mut midi_in = MidiInput::new("technus timecode reading midi timecode input")?;
        midi_in.ignore(Ignore::None);

        let port = self
            .port
            .take()
            .ok_or_else(|| anyhow!("No port specified, cannot open connection"))?;

        // Once port is taken for a connection, can no longer be in struct
        self.port = None;

        // Connect to the midi port and (for now) just print everything that comes through it
        let connection = midi_in
            .connect(
                &port,
                "technus-timecode-read-input",
                move |stamp, message, _| {
                    println!("{}: {:?} (len = {})", stamp, message, message.len());
                },
                (),
            )
            .map_err(|e| anyhow!("Failed to connect to MIDI port: {e}"))?;

        self.connection = Some(connection);

        Ok(())
    }

    /// Closes the currently open connection
    /// Result will be Err if there is no connection currently open
    fn close_connection(&mut self) -> Result<()> {
        let connection = self
            .connection
            .take()
            .ok_or_else(|| anyhow!("No connection is open, hence you cannot close it"))?;

        connection.close();

        // After closing the connection, it makes no sense to keep the old one around
        self.connection = None;

        Ok(())
    }
}

impl TimecodeDecoder for MtcTimecodeDecoder {
    fn get_current_timecode(&self) -> Timecode {
        self.current_timecode
    }
}
