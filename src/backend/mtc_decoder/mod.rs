use std::sync::mpsc::{self, Receiver};

use anyhow::{Result, anyhow};
use midi_msg::MidiMsg;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort};

use crate::backend::{
    mtc_decoder::timecode_quarter_frame_buffer::TimeCodeQuarterFrameBuffer, timecode::Timecode,
    timecode_decoder::TimecodeDecoder,
};

mod timecode_quarter_frame_buffer;

pub struct MtcTimecodeDecoder {
    pub fps: u8,
    pub port: Option<MidiInputPort>,
    timecode_rx: Option<Receiver<Timecode>>,
    connection: Option<MidiInputConnection<()>>,
    midi_in: MidiInput,
    connected: bool,
}

impl MtcTimecodeDecoder {
    pub fn new() -> Result<Self> {
        let mut midi_in = MidiInput::new("technus timecode reading input check ports")?;
        midi_in.ignore(Ignore::None);

        Ok(Self {
            fps: 0,
            port: None,
            timecode_rx: None,
            connection: None,
            midi_in,
            connected: false,
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
    pub fn connect(&mut self) -> Result<()> {
        let mut midi_in = MidiInput::new("technus timecode reading midi timecode input")?;
        midi_in.ignore(Ignore::None);

        let port = self
            .port
            .take()
            .ok_or_else(|| anyhow!("No port specified, cannot open connection"))?;

        // Once port is taken for a connection, can no longer be in struct
        self.port = None;

        // Create a new buffer for the received TimeCodeQuarterFrames
        let mut quarter_frame_buffer = TimeCodeQuarterFrameBuffer::new();

        // Create a channel to send the timecode values from the closure back to our function to get current timecode
        let (tx, rx) = mpsc::channel();
        self.timecode_rx = Some(rx);

        // Connect to the midi port and (for now) just print everything that comes through it
        let connection = midi_in
            .connect(
                &port,
                "technus-timecode-read-input",
                move |_, message, _| {
                    let (parsed_message, _) = match MidiMsg::from_midi(&message) {
                        Ok((msg, err)) => (msg, err),
                        Err(_) => return,
                    };

                    // Add the message to the TimeCodeQuarterFrameBuffer (ignores every message type
                    // other than TimeCodeQuarterFrameX)
                    quarter_frame_buffer.add(parsed_message);

                    // Construct a timecode if possible
                    let maybe_timecode = quarter_frame_buffer.construct_timecode();

                    // When we got a timecode, print it out
                    if let Some(tc) = maybe_timecode {
                        // Send the timecode
                        tx.send(tc.into()).expect("MTC Timecode pipe receiving end has been destroyed, cannot send timecode");
                    }
                },
                (),
            )
            .map_err(|e| anyhow!("Failed to connect to MIDI port: {e}"))?;

        self.connection = Some(connection);

        self.connected = true;

        Ok(())
    }

    /// Closes the currently open connection
    /// Result will be Err if there is no connection currently open
    pub fn disconnect(&mut self) -> Result<()> {
        let connection = self
            .connection
            .take()
            .ok_or_else(|| anyhow!("No connection is open, hence you cannot close it"))?;

        connection.close();

        self.connected = false;

        // After closing the connection, it makes no sense to keep the old one around
        self.connection = None;

        Ok(())
    }

    /// Returns true if there has been a connection made.
    pub fn connected(&self) -> bool {
        self.connected
    }
}

impl TimecodeDecoder for MtcTimecodeDecoder {
    /// Gets the current timecode value from MTC. If no new value is available then it returns None
    fn get_current_timecode(&mut self) -> Option<Timecode> {
        // Get the receiver, if we can't get it then just return the current timecode value
        let rx = match &self.timecode_rx {
            Some(rx) => rx,
            None => {
                return None;
            }
        };

        // Try to get data from the channel
        if let Ok(tc) = rx.try_recv() {
            return Some(tc);
        } else {
            return None;
        }
    }
}
