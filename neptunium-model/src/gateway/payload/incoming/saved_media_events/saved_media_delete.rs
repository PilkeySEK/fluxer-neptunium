use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedMediaDelete {
    #[serde(rename = "meme_id")]
    pub media_id: String,
}
