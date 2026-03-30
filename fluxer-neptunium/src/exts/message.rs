// TODO: Add some of the saved_media API here.

use async_trait::async_trait;
use neptunium_model::{
    channel::message::Message,
    id::{
        Id,
        marker::{AttachmentMarker, ChannelMarker, MessageMarker, UserMarker},
    },
};

use crate::{client::error::Error, events::context::Context};

use neptunium_http::endpoints::channel::{
    AddReaction, CreateMessage, CreateMessageBody, DeleteAllReactions, DeleteAllReactionsOfEmoji,
    DeleteMessage, DeleteMessageAttachment, DeleteOwnReaction, DeleteReaction, EditMessage,
    EditMessageBody, FetchMessage, MessageReference, PinMessage, UnpinMessage,
};

pub use neptunium_http::endpoints::channel::RequestReactionType as Reaction;

#[async_trait]
pub trait MessageExt {
    async fn reply(
        &self,
        ctx: &Context,
        message_body: impl Into<CreateMessageBody> + Send,
    ) -> Result<Message, Error>;

    async fn add_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error>;

    async fn delete_own_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error>;

    async fn delete_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
        target: Id<UserMarker>,
    ) -> Result<(), Error>;

    async fn delete_all_reactions_of_emoji(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error>;

    async fn delete_all_reactions(&self, ctx: &Context) -> Result<(), Error>;

    /// Delete this message.
    async fn delete(&self, ctx: &Context) -> Result<(), Error>;

    /// Edit this message.
    async fn edit(&self, ctx: &Context, updates: EditMessageBody) -> Result<Message, Error>;

    /// Re-fetches this message and returns the result.
    async fn fetch(&self, ctx: &Context) -> Result<Message, Error>;

    /// Mark the message as read.
    #[cfg(feature = "user_api")]
    async fn acknowledge(&self, ctx: &Context) -> Result<(), Error>;

    /// Mark the message as read.
    #[cfg(feature = "user_api")]
    async fn acknowledge_with_options(
        &self,
        ctx: &Context,
        mention_count: Option<u64>,
        manual: Option<bool>,
    ) -> Result<(), Error>;

    async fn delete_attachment(
        &self,
        ctx: &Context,
        attachment_id: Id<AttachmentMarker>,
    ) -> Result<(), Error>;

    async fn pin(&self, ctx: &Context) -> Result<(), Error>;
    async fn unpin(&self, ctx: &Context) -> Result<(), Error>;
    /// If this message mentions the current user, deletes it from the user's mention
    /// history. Note that this does not delete the message.
    #[cfg(feature = "user_api")]
    async fn delete_mention(&self, ctx: &Context) -> Result<(), Error>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply(
        &self,
        ctx: &Context,
        message_body: impl Into<CreateMessageBody> + Send,
    ) -> Result<Message, crate::client::error::Error> {
        let mut message_body = message_body.into();
        message_body.message_reference = Some(
            MessageReference::builder()
                .channel_id(self.channel_id)
                .message_id(self.id)
                .build(),
        );
        Ok(ctx
            .get_http_client()
            .execute(
                CreateMessage::builder()
                    .channel_id(self.channel_id)
                    .message(message_body)
                    .build(),
            )
            .await?)
    }

    async fn add_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), crate::client::error::Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                AddReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .build(),
            )
            .await?)
    }

    async fn delete_own_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteOwnReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .build(),
            )
            .await?)
    }

    async fn delete_reaction(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
        target: Id<UserMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteReaction::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .target(target)
                    .build(),
            )
            .await?)
    }

    async fn delete_all_reactions_of_emoji(
        &self,
        ctx: &Context,
        reaction: impl Into<Reaction<'_>> + Send,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteAllReactionsOfEmoji::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .reaction(&reaction.into())
                    .build(),
            )
            .await?)
    }

    async fn delete_all_reactions(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteAllReactions::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .build(),
            )
            .await?)
    }

    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteMessage::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .build(),
            )
            .await?)
    }

    async fn edit(&self, ctx: &Context, updates: EditMessageBody) -> Result<Message, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                EditMessage::builder()
                    .channel_id(self.channel_id)
                    .message_id(self.id)
                    .body(updates)
                    .build(),
            )
            .await?)
    }

    async fn fetch(&self, ctx: &Context) -> Result<Message, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                FetchMessage::builder()
                    .message_id(self.id)
                    .channel_id(self.channel_id)
                    .build(),
            )
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn acknowledge(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::channel::AcknowledgeMessage;

        Ok(ctx
            .get_http_client()
            .execute(
                AcknowledgeMessage::builder()
                    .message_id(self.id)
                    .channel_id(self.channel_id)
                    .build(),
            )
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn acknowledge_with_options(
        &self,
        ctx: &Context,
        mention_count: Option<u64>,
        manual: Option<bool>,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::channel::AcknowledgeMessage;

        Ok(ctx
            .get_http_client()
            .execute(AcknowledgeMessage {
                message_id: self.id,
                channel_id: self.channel_id,
                mention_count,
                manual,
            })
            .await?)
    }

    async fn delete_attachment(
        &self,
        ctx: &Context,
        attachment_id: Id<AttachmentMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteMessageAttachment {
                channel_id: self.channel_id,
                message_id: self.id,
                attachment_id,
            })
            .await?)
    }

    async fn pin(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(PinMessage {
                channel_id: self.channel_id,
                message_id: self.id,
            })
            .await?)
    }

    async fn unpin(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(UnpinMessage {
                channel_id: self.channel_id,
                message_id: self.id,
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn delete_mention(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::users::DeleteMention;

        Ok(ctx
            .get_http_client()
            .execute(DeleteMention {
                message_id: self.id,
            })
            .await?)
    }
}

#[async_trait]
pub trait MessageIdExt {
    async fn fetch(&self, ctx: &Context, channel_id: Id<ChannelMarker>) -> Result<Message, Error>;
}

#[async_trait]
impl MessageIdExt for Id<MessageMarker> {
    async fn fetch(&self, ctx: &Context, channel_id: Id<ChannelMarker>) -> Result<Message, Error> {
        Ok(ctx
            .get_http_client()
            .execute(FetchMessage {
                channel_id,
                message_id: *self,
            })
            .await?)
    }
}
