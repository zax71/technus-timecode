use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::app::GlobalState;
use crate::backend::timecode::Timecode;
use crate::backend::timecode_decoder::TimecodeDecoder;

#[derive(Default, Serialize, Deserialize)]
pub struct TimecodeDisplay {
    current_timecode: Timecode,
}

impl TimecodeDisplay {
    pub fn add(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, global_state: &mut GlobalState) {
        ui.label(format!("{}", self.current_timecode));

        // Update the cached timecode value if there is an update
        if let Some(tc) = global_state.mtc_decoder.get_current_timecode() {
            self.current_timecode = tc;
            ctx.request_repaint(); // Keep the timecode updating even when there is no user input
        }
    }
}
