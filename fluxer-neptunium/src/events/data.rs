use std::ops::{Deref, DerefMut};

use debug_ignore::DebugIgnore;
use fluxer_gateway::model::{
    event::dispatch::{channel::MessageCreateDispatchData, session::ReadyDispatchData},
    snowflake::Snowflake,
};

use crate::{
    ApiClientMessage, ClientInfo, ClientMessage,
    api::messages::{MessageReference, MessageReferenceType},
};

#[derive(Clone, Debug)]
pub struct ReadyEventData {
    pub dispatch_data: ReadyDispatchData,
}

impl Deref for ReadyEventData {
    type Target = ReadyDispatchData;
    fn deref(&self) -> &Self::Target {
        &self.dispatch_data
    }
}

impl DerefMut for ReadyEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dispatch_data
    }
}

#[derive(Clone, Debug)]
pub struct MessageCreateEventData {
    pub dispatch_data: MessageCreateDispatchData,
    pub(crate) client_info: DebugIgnore<ClientInfo>,
}

impl Deref for MessageCreateEventData {
    type Target = MessageCreateDispatchData;
    fn deref(&self) -> &Self::Target {
        &self.dispatch_data
    }
}

impl DerefMut for MessageCreateEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dispatch_data
    }
}

impl MessageCreateEventData {
    // WIP
    pub fn reply(&self, content: String) {
        let _ = self.client_info.tx.send(ClientMessage::ApiClientMessage(
            ApiClientMessage::SendMessage {
                channel_id: self.dispatch_data.message_response.channel_id.clone(),
                content,
                reference: Some(MessageReference {
                    channel_id: self.message_response.channel_id.clone(),
                    message_id: self.message_response.id.clone(),
                    guild_id: None,
                    r#type: MessageReferenceType::Reply,
                }),
            },
        ));
    }
}

#[derive(Clone, Debug)]
pub struct GuildDeleteEventData {
    pub id: Snowflake,
    pub unavailable: bool,
}

#[derive(Clone, Debug)]
pub struct GuildCreateEventData {}
