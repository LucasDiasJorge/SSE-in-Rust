use serde::{Deserialize, Serialize};
use serde_json::{self, Number};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct EventPayload {
    pub(crate) id: i16,
    pub(crate) timestamp: Number,
    pub(crate) message: String,
}

impl EventPayload {
    pub(crate) fn serialize(&self) -> String {
        serde_json::to_string(self)
            .map(|json| format!("{}\n", json))
            .unwrap_or_else(|_| String::from("{}\n"))
    }
}
