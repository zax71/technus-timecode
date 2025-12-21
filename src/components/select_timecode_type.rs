use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimecodeType {
    Artnet,
    Ltc,
    Mtc,
}

impl fmt::Display for TimecodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Artnet => write!(f, "Art-Net"),
            Self::Ltc => write!(f, "LTC"),
            Self::Mtc => write!(f, "MTC"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectTimecodeType {
    pub current_type: TimecodeType,
}

impl SelectTimecodeType {
    pub fn new() -> Self {
        Self {
            current_type: TimecodeType::Mtc,
        }
    }

    pub fn add(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Timecode type")
            .selected_text(format!("{}", self.current_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.current_type,
                    TimecodeType::Artnet,
                    format!("{}", TimecodeType::Artnet),
                );
                ui.selectable_value(
                    &mut self.current_type,
                    TimecodeType::Ltc,
                    format!("{}", TimecodeType::Ltc),
                );
                ui.selectable_value(
                    &mut self.current_type,
                    TimecodeType::Mtc,
                    format!("{}", TimecodeType::Mtc),
                );
            });
    }
}
