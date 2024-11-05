use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct EventPayload {
    pub(crate) id: i16,
    pub(crate) timestamp: Number,
    pub(crate) message: String
}
