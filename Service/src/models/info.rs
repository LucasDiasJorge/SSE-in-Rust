use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Info {
    pub(crate) message: String,
}
