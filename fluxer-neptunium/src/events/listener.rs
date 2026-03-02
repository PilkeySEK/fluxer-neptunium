// macro_rules! event_listener {
//     (
//         $(
//             $(#[$meta:meta])*
//             async fn $fn_name:ident ($($arg_name:ident : $arg_type:ty)*)
//         )*
//     ) => {
//
//     };
// }

use async_trait::async_trait;

use crate::ClientInfo;
use crate::events::data::{
    GuildCreateEventData, GuildDeleteEventData, MessageCreateEventData, ReadyEventData,
};

#[async_trait]
pub trait EventListener {
    #[expect(unused)]
    #[inline]
    async fn ready(&mut self, ctx: &Context, data: ReadyEventData) {}

    #[expect(unused)]
    #[inline]
    async fn message_create(&mut self, ctx: &Context, data: MessageCreateEventData) {}

    #[expect(unused)]
    #[inline]
    async fn guild_delete(&mut self, ctx: &Context, data: GuildDeleteEventData) {}

    #[expect(unused)]
    #[inline]
    async fn guild_create(&mut self, ctx: &Context, data: GuildCreateEventData) {}
}

pub struct Context {
    #[expect(unused)]
    pub(crate) client_info: ClientInfo,
}
