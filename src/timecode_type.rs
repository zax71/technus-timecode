use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimecodeType {
    Artnet,
    Ltc,
    Mtc,
}

impl Default for TimecodeType {
    fn default() -> Self {
        TimecodeType::Mtc
    }
}
