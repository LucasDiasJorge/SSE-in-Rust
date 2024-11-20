use serde::{Deserialize, Serialize};
use serde_json::{self, Number};

//data:{"id":-1,"title":"HANDSHAKE","message":"OK","event":null,"content":null,"isRead":false,"company":null}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Content{
    pub(crate) epc: String,
    pub(crate) timestamp: Number,
    pub(crate) newLocation: String,
    pub(crate) lastLocation: String,
    pub(crate) sku: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct EventPayload {
    pub(crate) id: i16,
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) event: String,
    pub(crate) content: Content,
    pub(crate) timestamp: Number,
    pub(crate) isRead: bool,
    pub(crate) company: String,
}

impl EventPayload {
    pub(crate) fn serialize(&self) -> String {
        serde_json::to_string(self)
            .map(|json| format!("{}\n", json))
            .unwrap_or_else(|_| String::from("{}\n"))
    }
}
