use midi_msg::{MidiMsg, SystemCommonMsg, TimeCode};

/// A buffer for TimeCodeQuarterFrameX from which a full TimeCode can
/// be constructed when all 8 Quartes have been received
/// Full credit to: https://github.com/AlexCharlton/midi-msg/blob/master/examples/read_mtc_from_input.rs
pub struct TimeCodeQuarterFrameBuffer {
    buffer: [Option<TimeCode>; 8],
}

impl TimeCodeQuarterFrameBuffer {
    /// Return a new empty TimeCodeQuarterFrameBuffer
    pub fn new() -> Self {
        Self { buffer: [None; 8] }
    }

    /// Add a TimeCodeQuarterFrameX to the buffer, replacing the old one
    pub fn add(&mut self, message: MidiMsg) {
        if let MidiMsg::SystemCommon { msg } = message {
            // Get the target index and timecode from the TimeCodeQuarterFrameX
            let (index, tc) = match msg {
                SystemCommonMsg::TimeCodeQuarterFrame1(tc) => (0, tc),
                SystemCommonMsg::TimeCodeQuarterFrame2(tc) => (1, tc),
                SystemCommonMsg::TimeCodeQuarterFrame3(tc) => (2, tc),
                SystemCommonMsg::TimeCodeQuarterFrame4(tc) => (3, tc),
                SystemCommonMsg::TimeCodeQuarterFrame5(tc) => (4, tc),
                SystemCommonMsg::TimeCodeQuarterFrame6(tc) => (5, tc),
                SystemCommonMsg::TimeCodeQuarterFrame7(tc) => (6, tc),
                SystemCommonMsg::TimeCodeQuarterFrame8(tc) => (7, tc),
                _ => return,
            };
            // Store the fitting tc at the matching position if is None
            if self.buffer[index].is_none() {
                self.buffer[index] = Some(tc);
            }
        }
    }

    /// Return true if all values in the buffer have a value, false otherwise
    fn is_filled(&self) -> bool {
        self.buffer.iter().all(|b| b.is_some())
    }

    /// Construct a Timecode from the TimeCodeQuarterFrames if possible
    pub fn construct_timecode(&mut self) -> Option<TimeCode> {
        // If the Buffer is not ready, return None
        if !self.is_filled() {
            return None;
        }
        // Combine the 4 bit nibbles of the pairs of TimeCode
        // E.g. the low nibble of the frames: u8 stored in TimeCodeQuarter1
        // and the high nibble of frames u8 stored in TimeCodeQuarter2
        let frames: u8 = self.buffer[0].unwrap().frames ^ self.buffer[1].unwrap().frames;
        let seconds: u8 = self.buffer[2].unwrap().seconds ^ self.buffer[3].unwrap().seconds;
        let minutes: u8 = self.buffer[4].unwrap().minutes ^ self.buffer[5].unwrap().minutes;
        let hours: u8 = self.buffer[6].unwrap().hours ^ self.buffer[7].unwrap().hours;
        // The last high TimeCodeQuarter contains the propper code_type so extract it from there
        let code_type = self.buffer[7].unwrap().code_type;
        // Empty the buffer
        self.buffer = [None; 8];
        // Construct and return the TimeCode
        Some(TimeCode {
            frames,
            seconds,
            minutes,
            hours,
            code_type,
        })
    }
}
