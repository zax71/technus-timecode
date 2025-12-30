use crate::backend::timecode::Timecode;

pub trait TimecodeDecoder {
    fn get_current_timecode(&mut self) -> Option<Timecode>;
}
