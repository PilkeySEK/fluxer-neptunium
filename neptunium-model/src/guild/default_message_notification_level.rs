use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    AllMessages = 0,
    MentionsOnly = 1,
}
