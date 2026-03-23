use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::UserMarker},
    misc::{HexColor32, ImageHash},
    user::flags::PublicUserFlags,
};

#[cfg(feature = "user_api")]
pub mod auth;
pub mod flags;
pub mod read_state;
pub mod relationship;
pub mod settings;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPartial {
    pub avatar: Option<ImageHash>,
    pub avatar_color: Option<HexColor32>,
    #[serde(default)]
    pub bot: bool,
    // TODO: Maybe store as u16 instead
    pub discriminator: String,
    pub flags: PublicUserFlags,
    pub global_name: Option<String>,
    pub id: Id<UserMarker>,
    #[serde(default)]
    pub system: bool,
    /// Note that this is not unique (because Fluxer has discriminators)
    pub username: String,
}
