use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TimecodeType {
    ARTNET,
    LTC,
    MTC,
}

impl fmt::Display for TimecodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ARTNET => write!(f, "Art-Net"),
            Self::LTC => write!(f, "LTC"),
            Self::MTC => write!(f, "MTC"),
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
            current_type: TimecodeType::MTC,
        }
    }

    pub fn add(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Timecode type")
            .selected_text(format!("{}", self.current_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.current_type,
                    TimecodeType::ARTNET,
                    format!("{}", TimecodeType::ARTNET),
                );
                ui.selectable_value(
                    &mut self.current_type,
                    TimecodeType::LTC,
                    format!("{}", TimecodeType::LTC),
                );
                ui.selectable_value(
                    &mut self.current_type,
                    TimecodeType::MTC,
                    format!("{}", TimecodeType::MTC),
                );
            });
    }
}
