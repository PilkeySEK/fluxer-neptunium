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
    GenericMarker,
    EmojiMarker,
    ApplicationMarker,
    StickerMarker,
    RoleMarker,
    WebhookMarker,
    AuditLogEntryMarker,
    PackMarker,
    RelationshipMarker,
    AttachmentMarker,
}

pub trait IdMarker: Copy + Clone + std::hash::Hash + PartialEq + Eq + std::fmt::Debug {}
