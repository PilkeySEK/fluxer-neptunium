use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    gateway::presence::CustomStatus,
    time::timestamp::{Timestamp, representations::UnixMillis},
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Online,
    #[serde(rename = "dnd")]
    DoNotDisturb,
    Idle,
    Invisible,
    Offline,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
pub struct PresenceUpdateOutgoing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<Timestamp<UnixMillis>>,
    // #[builder(default = vec![])]
    // pub activities: Vec<Activity>,
    #[builder(default = Status::Online)]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_status: Option<CustomStatus>,
    #[builder(default = false)]
    pub afk: bool,
    #[builder(default = false)]
    pub mobile: bool,
}
