//! All of the marker structs.

macro_rules! marker {
    ($($(#[$meta:meta])* $name:ident),* $(,)?) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $name;
            impl IdMarker for $name {}
        )*
    };
}

marker! {
    UserMarker,
    GuildMarker,
    ChannelMarker,
    MessageMarker,
    VoiceRegionMarker,
    GenericMarker,
    EmojiMarker,
    ApplicationMarker,
    StickerMarker,
    RoleMarker,
    WebhookMarker,
    AttachmentMarker,
    AuditLogEntryMarker,
    PackMarker,
    RelationshipMarker,
}

pub trait IdMarker: Copy + Clone + std::hash::Hash + PartialEq + Eq + std::fmt::Debug {}
