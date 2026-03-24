use serde::Deserialize;

use crate::{
    channel::Channel,
    gateway::{presence::Presence, voice_state::VoiceState},
    guild::{Guild, member::GuildMember},
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildCreate {
    #[serde(flatten)]
    pub guild: Guild,
    pub channels: Vec<Channel>,
    pub members: Vec<GuildMember>,
    pub presences: Vec<Presence>,
    pub voice_states: Vec<VoiceState>,
    pub joined_at: Timestamp<Iso8601>,
}
