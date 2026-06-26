use bitflags::bitflags;

use crate::misc::serde_bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct PublicUserFlags: u64 {
        /// User is a staff member.
        const STAFF = 1 << 0;
        /// User is a CTP member.
        const CTP_MEMBER = 1 << 1;
        /// User is a partner.
        const PARTNER = 1 << 2;
        /// User is a bug hunter.
        const BUG_HUNTER = 1 << 3;
        /// User has elevated global rate limits.
        const HIGH_GLOBAL_RATE_LIMIT = 1 << 33;
        /// Bot accepts friend requests from users.
        const FRIENDLY_BOT = 1 << 4;
        /// Bot requires manual approval for friend requests.
        const FRIENDLY_BOT_MANUAL_APPROVAL = 1 << 5;
        /// User is flagged as a spammer.
        const SPAMMER = 1 << 6;
        /// User account has been deleted.
        const DELETED = 1 << 34;
        /// User account disabled due to suspicious activity.
        const DISABLED_SUSPICIOUS_ACTIVITY = 1 << 35;
        /// User account was self-deleted.
        const SELF_DELETED = 1 << 36;
        /// User account is disabled.
        const DISABLED = 1 << 38;
        /// User has started a session.
        const HAS_SESSION_STARTED = 1 << 39;
        /// User can bypass rate limits.
        const RATE_LIMIT_BYPASS = 1 << 47;
        /// User is banned from reporting.
        const REPORT_BANNED = 1 << 48;
        /// User is verified as not underage.
        const VERIFIED_NOT_UNDERAGE = 1 << 49;
        /// User has dismissed premium onboarding.
        const HAS_DISMISSED_PREMIUM_ONBOARDING = 1 << 51;
        /// User is an app store reviewer.
        const APP_STORE_REVIEWER = 1 << 53;
        /// User staff status is hidden from public flags.
        const STAFF_HIDDEN = 1 << 57;
        /// User has verified their age as an adult via credit card verification.
        const AGE_VERIFIED_ADULT = 1 << 60;
        /// User is forced through inbound (expensive-destination) phone verification regardless of phone prefix, for debugging.
        const FORCE_INBOUND_PHONE_VERIFICATION = 1 << 61;
        /// User is permanently exempt from automatic suspicious-activity flagging on RPC session start (does not require a prior payment).
        const NOT_SUSPICIOUS = 1 << 62;
    }
}

serde_bitflags! {PublicUserFlags, u64}
