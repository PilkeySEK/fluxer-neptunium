use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedMediaDelete {
    pub meme_id: String,
}
