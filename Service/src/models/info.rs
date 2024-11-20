use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Info {
    pub(crate) event_type: String,
}
