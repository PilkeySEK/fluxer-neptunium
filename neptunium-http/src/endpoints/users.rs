#[cfg(feature = "user_api")]
mod accept_or_update_friend_request;
#[cfg(feature = "user_api")]
mod add_phone_number_to_account;
#[cfg(feature = "user_api")]
mod cancel_bulk_message_deletion;
#[cfg(feature = "user_api")]
mod check_username_tag_availability;
#[cfg(feature = "user_api")]
mod complete_password_change;
#[cfg(feature = "user_api")]
mod delete_current_user_account;
#[cfg(feature = "user_api")]
mod delete_current_user_mention;
#[cfg(feature = "user_api")]
mod delete_webauthn_credential;
#[cfg(feature = "user_api")]
mod disable_current_user_account;
#[cfg(feature = "user_api")]
mod disable_sms_mfa;
#[cfg(feature = "user_api")]
mod disable_totp_mfa;
#[cfg(feature = "user_api")]
mod enable_sms_mfa;
#[cfg(feature = "user_api")]
mod enable_totp_mfa;
#[cfg(feature = "user_api")]
mod forget_authorized_ips_for_current_user;
#[cfg(feature = "user_api")]
mod get_backup_codes_for_mfa;
mod get_current_user_profile;
#[cfg(feature = "user_api")]
mod get_current_user_settings;
#[cfg(feature = "user_api")]
mod get_data_harvest_download_url;
#[cfg(feature = "user_api")]
mod get_data_harvest_status;
#[cfg(feature = "user_api")]
mod get_latest_data_harvest;
#[cfg(feature = "user_api")]
mod get_note_on_user;
#[cfg(feature = "user_api")]
mod get_sudo_webauthn_authentication_options;
mod get_user_by_id;
mod get_user_profile;
#[cfg(feature = "user_api")]
mod get_webauthn_registration_options;
#[cfg(feature = "user_api")]
mod list_current_user_mentions;
#[cfg(feature = "user_api")]
mod list_current_user_notes;
#[cfg(feature = "user_api")]
mod list_push_subscriptions;
#[cfg(feature = "user_api")]
mod list_relationships;
#[cfg(feature = "user_api")]
mod list_sudo_mfa_authentication_methods;
#[cfg(feature = "user_api")]
mod list_user_gifts;
#[cfg(feature = "user_api")]
mod list_webauthn_credentials;
#[cfg(feature = "user_api")]
mod register_webauthn_credential;
#[cfg(feature = "user_api")]
mod remove_phone_number_from_account;
#[cfg(feature = "user_api")]
mod remove_relationship;
#[cfg(feature = "user_api")]
mod request_bulk_message_deletion;
#[cfg(feature = "user_api")]
mod request_data_harvest;
#[cfg(feature = "user_api")]
mod request_new_email_address;
#[cfg(feature = "user_api")]
mod request_replacement_email_for_bounced_address;
#[cfg(feature = "user_api")]
mod resend_new_email_confirmation;
#[cfg(feature = "user_api")]
mod resend_original_email_confirmation;
#[cfg(feature = "user_api")]
mod resend_password_change_verification_code;
#[cfg(feature = "user_api")]
mod resend_replacement_email_code;
#[cfg(feature = "staff_api")]
mod reset_current_user_premium_state;
#[cfg(feature = "user_api")]
mod send_friend_request;
#[cfg(feature = "user_api")]
mod send_friend_request_by_tag;
#[cfg(feature = "user_api")]
mod send_phone_verification_code;
#[cfg(feature = "user_api")]
mod send_sudo_sms_code;
#[cfg(feature = "user_api")]
mod set_note_on_user;
#[cfg(feature = "user_api")]
mod start_email_change;
#[cfg(feature = "user_api")]
mod start_password_change;
#[cfg(feature = "user_api")]
mod subscribe_to_push_notifications;
#[cfg(feature = "staff_api")]
mod test_bulk_message_deletion;
#[cfg(feature = "user_api")]
mod unsubscribe_from_push_notifications;
#[cfg(feature = "user_api")]
mod update_current_user_profile;
#[cfg(feature = "user_api")]
mod update_current_user_settings;
#[cfg(feature = "user_api")]
mod update_dm_notification_settings;
#[cfg(feature = "user_api")]
mod update_guild_settings_for_user;
#[cfg(feature = "user_api")]
mod update_relationship_nickname;
#[cfg(feature = "user_api")]
mod update_webauthn_credential;
#[cfg(feature = "user_api")]
mod verify_new_email_address;
#[cfg(feature = "user_api")]
mod verify_original_email_address;
#[cfg(feature = "user_api")]
mod verify_password_change_code;
#[cfg(feature = "user_api")]
mod verify_phone_code;
#[cfg(feature = "user_api")]
mod verify_replacement_email_for_bounced_address;

#[cfg(feature = "user_api")]
pub use accept_or_update_friend_request::*;
#[cfg(feature = "user_api")]
pub use add_phone_number_to_account::*;
#[cfg(feature = "user_api")]
pub use cancel_bulk_message_deletion::*;
#[cfg(feature = "user_api")]
pub use check_username_tag_availability::*;
#[cfg(feature = "user_api")]
pub use complete_password_change::*;
#[cfg(feature = "user_api")]
pub use delete_current_user_account::*;
#[cfg(feature = "user_api")]
pub use delete_current_user_mention::*;
#[cfg(feature = "user_api")]
pub use delete_webauthn_credential::*;
#[cfg(feature = "user_api")]
pub use disable_current_user_account::*;
#[cfg(feature = "user_api")]
pub use disable_sms_mfa::*;
#[cfg(feature = "user_api")]
pub use disable_totp_mfa::*;
#[cfg(feature = "user_api")]
pub use enable_sms_mfa::*;
#[cfg(feature = "user_api")]
pub use enable_totp_mfa::*;
#[cfg(feature = "user_api")]
pub use forget_authorized_ips_for_current_user::*;
#[cfg(feature = "user_api")]
pub use get_backup_codes_for_mfa::*;
pub use get_current_user_profile::*;
#[cfg(feature = "user_api")]
pub use get_current_user_settings::*;
#[cfg(feature = "user_api")]
pub use get_data_harvest_download_url::*;
#[cfg(feature = "user_api")]
pub use get_data_harvest_status::*;
#[cfg(feature = "user_api")]
pub use get_latest_data_harvest::*;
#[cfg(feature = "user_api")]
pub use get_note_on_user::*;
#[cfg(feature = "user_api")]
pub use get_sudo_webauthn_authentication_options::*;
pub use get_user_by_id::*;
pub use get_user_profile::*;
#[cfg(feature = "user_api")]
pub use get_webauthn_registration_options::*;
#[cfg(feature = "user_api")]
pub use list_current_user_mentions::*;
#[cfg(feature = "user_api")]
pub use list_current_user_notes::*;
#[cfg(feature = "user_api")]
pub use list_push_subscriptions::*;
#[cfg(feature = "user_api")]
pub use list_relationships::*;
#[cfg(feature = "user_api")]
pub use list_sudo_mfa_authentication_methods::*;
#[cfg(feature = "user_api")]
pub use list_user_gifts::*;
#[cfg(feature = "user_api")]
pub use list_webauthn_credentials::*;
#[cfg(feature = "user_api")]
pub use register_webauthn_credential::*;
#[cfg(feature = "user_api")]
pub use remove_phone_number_from_account::*;
#[cfg(feature = "user_api")]
pub use remove_relationship::*;
#[cfg(feature = "user_api")]
pub use request_bulk_message_deletion::*;
#[cfg(feature = "user_api")]
pub use request_data_harvest::*;
#[cfg(feature = "user_api")]
pub use request_new_email_address::*;
#[cfg(feature = "user_api")]
pub use request_replacement_email_for_bounced_address::*;
#[cfg(feature = "user_api")]
pub use resend_new_email_confirmation::*;
#[cfg(feature = "user_api")]
pub use resend_original_email_confirmation::*;
#[cfg(feature = "user_api")]
pub use resend_password_change_verification_code::*;
#[cfg(feature = "user_api")]
pub use resend_replacement_email_code::*;
#[cfg(feature = "staff_api")]
pub use reset_current_user_premium_state::*;
#[cfg(feature = "user_api")]
pub use send_friend_request::*;
#[cfg(feature = "user_api")]
pub use send_friend_request_by_tag::*;
#[cfg(feature = "user_api")]
pub use send_phone_verification_code::*;
#[cfg(feature = "user_api")]
pub use send_sudo_sms_code::*;
#[cfg(feature = "user_api")]
pub use set_note_on_user::*;
#[cfg(feature = "user_api")]
pub use start_email_change::*;
#[cfg(feature = "user_api")]
pub use start_password_change::*;
#[cfg(feature = "user_api")]
pub use subscribe_to_push_notifications::*;
#[cfg(feature = "staff_api")]
pub use test_bulk_message_deletion::*;
#[cfg(feature = "user_api")]
pub use unsubscribe_from_push_notifications::*;
#[cfg(feature = "user_api")]
pub use update_current_user_profile::*;
#[cfg(feature = "user_api")]
pub use update_current_user_settings::*;
#[cfg(feature = "user_api")]
pub use update_dm_notification_settings::*;
#[cfg(feature = "user_api")]
pub use update_guild_settings_for_user::*;
#[cfg(feature = "user_api")]
pub use update_relationship_nickname::*;
#[cfg(feature = "user_api")]
pub use update_webauthn_credential::*;
#[cfg(feature = "user_api")]
pub use verify_new_email_address::*;
#[cfg(feature = "user_api")]
pub use verify_original_email_address::*;
#[cfg(feature = "user_api")]
pub use verify_password_change_code::*;
#[cfg(feature = "user_api")]
pub use verify_phone_code::*;
#[cfg(feature = "user_api")]
pub use verify_replacement_email_for_bounced_address::*;
