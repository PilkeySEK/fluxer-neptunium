#[cfg(feature = "user_api")]
mod cancel_bulk_message_deletion;
#[cfg(feature = "user_api")]
mod delete_current_user_account;
#[cfg(feature = "user_api")]
mod delete_current_user_mention;
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
mod get_data_harvest_download_url;
#[cfg(feature = "user_api")]
mod get_data_harvest_status;
#[cfg(feature = "user_api")]
mod get_latest_data_harvest;
#[cfg(feature = "user_api")]
mod list_current_user_mentions;
#[cfg(feature = "user_api")]
mod list_user_gifts;
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
mod resend_replacement_email_code;
#[cfg(feature = "user_api")]
mod start_email_change;
#[cfg(feature = "staff_api")]
mod test_bulk_message_deletion;
#[cfg(feature = "user_api")]
mod update_current_user_profile;
#[cfg(feature = "user_api")]
mod update_dm_notification_settings;
#[cfg(feature = "user_api")]
mod update_guild_settings_for_user;
#[cfg(feature = "user_api")]
mod verify_new_email_address;
#[cfg(feature = "user_api")]
mod verify_original_email_address;
#[cfg(feature = "user_api")]
mod verify_replacement_email_for_bounced_address;

#[cfg(feature = "user_api")]
pub use cancel_bulk_message_deletion::*;
#[cfg(feature = "user_api")]
pub use delete_current_user_account::*;
#[cfg(feature = "user_api")]
pub use delete_current_user_mention::*;
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
pub use get_data_harvest_download_url::*;
#[cfg(feature = "user_api")]
pub use get_data_harvest_status::*;
#[cfg(feature = "user_api")]
pub use get_latest_data_harvest::*;
#[cfg(feature = "user_api")]
pub use list_current_user_mentions::*;
#[cfg(feature = "user_api")]
pub use list_user_gifts::*;
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
pub use resend_replacement_email_code::*;
#[cfg(feature = "user_api")]
pub use start_email_change::*;
#[cfg(feature = "staff_api")]
pub use test_bulk_message_deletion::*;
#[cfg(feature = "user_api")]
pub use update_current_user_profile::*;
#[cfg(feature = "user_api")]
pub use update_dm_notification_settings::*;
#[cfg(feature = "user_api")]
pub use update_guild_settings_for_user::*;
#[cfg(feature = "user_api")]
pub use verify_new_email_address::*;
#[cfg(feature = "user_api")]
pub use verify_original_email_address::*;
#[cfg(feature = "user_api")]
pub use verify_replacement_email_for_bounced_address::*;
