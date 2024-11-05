use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct HandshakePayload {
    pub(crate) id: u64,
    pub(crate) event: String,
}
