use std::fmt::{Display, write};

/// Stores timecode values with an associated FPS, can be used for SMPTE timecode and others.
/// Values are private to enforce the following conditions:
///     minutes < 60
///     seconds < 60
///     frames < fps
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Timecode {
    hours: u8,
    minutes: u8,
    seconds: u8,
    frames: u8,
    fps: u8,
}

impl Timecode {
    /// Creates a new timecode struct, it will automatically correct for when values are too high.
    /// E.g: if seconds are > 60 then it wil roll that over to minutes
    pub fn new(mut hours: u8, mut minutes: u8, mut seconds: u8, mut frames: u8, fps: u8) -> Self {
        // If the number of frames is too high, convert the extra to seconds and add the remainder to the frames
        if frames >= fps {
            seconds = seconds + frames / fps;
            frames = frames % fps;
        }

        // The value at which we should roll over to the next value for minutes and seconds
        let max_time = 60;

        // If there are too many seconds and it could be stored in minutes, do so
        if seconds >= max_time {
            minutes = minutes + seconds / max_time;
            seconds = seconds % max_time;
        }

        // If there are too many minutes and it should be stored in hours, do so
        if minutes >= max_time {
            hours = hours + minutes / max_time;
            minutes = minutes % max_time;
        }

        // Now that we are sure that the timecode value makes sense, store it to the struct
        Self {
            hours,
            minutes,
            seconds,
            frames,
            fps,
        }
    }
}

impl Display for Timecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{} @ {}fps",
            self.hours, self.minutes, self.seconds, self.frames, self.fps
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Tests for when the frames value is normal

    #[test]
    fn test_boundary_hours_unchanged() {
        let timecode = Timecode::new(100, 59, 59, 23, 24);

        assert_eq!(timecode.hours, 100)
    }

    #[test]
    fn test_boundary_minutes_unchanged() {
        let timecode = Timecode::new(100, 59, 59, 23, 24);

        assert_eq!(timecode.minutes, 59)
    }

    #[test]
    fn test_boundary_seconds_unchanged() {
        let timecode = Timecode::new(100, 59, 59, 23, 24);

        assert_eq!(timecode.seconds, 59)
    }

    #[test]
    fn test_boundary_frames_unchanged() {
        let timecode = Timecode::new(100, 59, 59, 23, 24);

        assert_eq!(timecode.frames, 23)
    }

    // Tests for when the frames value is too high

    #[test]
    fn test_boundary_over_change_frames() {
        let timecode = Timecode::new(100, 59, 59, 25, 24);

        println!("Hours unchanged timecode: {}", timecode);

        assert_eq!(timecode.frames, 1)
    }

    #[test]
    fn test_boundary_over_change_seconds() {
        let timecode = Timecode::new(100, 59, 59, 25, 24);

        assert_eq!(timecode.seconds, 0)
    }

    #[test]
    fn test_boundary_over_change_minutes() {
        let timecode = Timecode::new(100, 59, 59, 25, 24);

        assert_eq!(timecode.minutes, 0)
    }

    #[test]
    fn test_boundary_over_change_hours() {
        let timecode = Timecode::new(100, 59, 59, 25, 24);

        println!("Hours unchanged timecode: {}", timecode);

        assert_eq!(timecode.hours, 101)
    }
}
