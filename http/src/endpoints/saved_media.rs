// Source: https://fluxer.app/channels/1427764813854588940/1483532018185537313/1485014094289854713 (hopefully accessible)
//! Saved media, also often referred to as "favorite meme" in the Fluxer API docs.
//! The names of structs here have been changed to be more consistently named (context: saved media was originally
//! called "favorite memes", but this was changed, the API doesn't reflect the new naming (yet?) though).

pub mod delete_saved_media;
pub mod get_saved_media;
pub mod list_saved_media;
pub mod save_message_attachment;
pub mod save_url;
pub mod update_saved_media;
