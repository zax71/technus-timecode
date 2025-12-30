use core::fmt;
use std::fmt::format;

use egui::DragValue;
use serde::{Deserialize, Serialize};

use crate::{
    app::GlobalState, backend::mtc_decoder::MtcTimecodeDecoder, timecode_type::TimecodeType,
};

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

impl Default for SelectTimecodeType {
    fn default() -> Self {
        Self {
            current_type: TimecodeType::Mtc,
        }
    }
}

impl SelectTimecodeType {
    pub fn add(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, global_state: &mut GlobalState) {
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

        match self.current_type {
            TimecodeType::Mtc => self.add_mtc(ctx, ui, global_state),
            _ => println!("Art-Net and LTC are not implemented yet"),
        };
    }

    /// Displays the UI elements to select the MIDI device & FPS
    fn add_mtc(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, global_state: &mut GlobalState) {
        // Gets the name of the currently selected port. Will display appropriately if there are no available ports,
        // and will throw a toast if initialising MIDI support failed
        let currently_selected_port_name = match &global_state.mtc_decoder.port {
            Some(port) => match global_state.mtc_decoder.port_name(port) {
                Ok(p) => p,
                Err(e) => {
                    global_state
                        .toasts
                        .error(format!("Failed to initialise MIDI to get port names: {e}"));
                    return;
                }
            },
            None => "Select a port".to_string(),
        };

        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Select MIDI Device")
                .selected_text(currently_selected_port_name)
                .show_ui(ui, |ui| {
                    for current_port in global_state.mtc_decoder.get_ports() {
                        let current_port_name =
                            match global_state.mtc_decoder.port_name(&current_port) {
                                Ok(p) => p,
                                Err(e) => {
                                    global_state.toasts.error(format!(
                                        "Failed to initialise MIDI to get port names: {e}"
                                    ));
                                    return;
                                }
                            };

                        ui.selectable_value(
                            &mut global_state.mtc_decoder.port,
                            Some(current_port),
                            current_port_name,
                        );
                    }
                });

            ui.label("FPS:");
            ui.add(DragValue::new(&mut global_state.mtc_decoder.fps));

            let connect_button_text: &str = match global_state.mtc_decoder.connected() {
                true => "Disconnect",
                false => "Connect",
            };

            if ui.button(connect_button_text).clicked() {
                // Connect to the aforementioned MIDI ports, or throw a toast error
                if !global_state.mtc_decoder.connected() {
                    match global_state.mtc_decoder.connect() {
                        Ok(_) => {
                            global_state.toasts.success(format!("Connected"));
                        }
                        Err(e) => {
                            global_state
                                .toasts
                                .error(format!("Failed to connect to MTC timecode: {e}"));
                        }
                    }
                // Disconnect from the aforementioned MIDI ports, error *should* be unreachable
                } else {
                    match global_state.mtc_decoder.disconnect() {
                        Ok(_) => {
                            global_state.toasts.info("Disconnected");
                        }
                        Err(e) => {
                            global_state
                                .toasts
                                .error(format!("Failed to disconnect: {e}"));
                        }
                    }
                }
            }
        });
    }
}
