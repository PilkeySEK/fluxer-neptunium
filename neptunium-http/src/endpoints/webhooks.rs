mod create_webhook;
mod delete_webhook;
mod delete_webhook_message;
mod delete_webhook_with_token;
mod edit_webhook_message;
mod execute_webhook;
mod get_webhook;
mod get_webhook_with_token;
mod list_channel_webhooks;
mod list_guild_webhooks;
mod update_webhook;
mod update_webhook_with_token;

pub use create_webhook::*;
pub use delete_webhook::*;
pub use delete_webhook_message::*;
pub use delete_webhook_with_token::*;
pub use edit_webhook_message::*;
pub use execute_webhook::*;
pub use get_webhook::*;
pub use get_webhook_with_token::*;
pub use list_channel_webhooks::*;
pub use list_guild_webhooks::*;
pub use update_webhook::*;
pub use update_webhook_with_token::*;

// TODO: Execute GitHub, Sentry, and Slack webhook endpoints:
// https://docs.fluxer.app/api-reference/webhooks/execute-github-webhook
// https://docs.fluxer.app/api-reference/webhooks/execute-sentry-webhook
// https://docs.fluxer.app/api-reference/webhooks/execute-slack-webhook
