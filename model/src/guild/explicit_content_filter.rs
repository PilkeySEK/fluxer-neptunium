use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    None = 0,
    MembersWithoutRole = 1,
    AllMembers = 2,
}
