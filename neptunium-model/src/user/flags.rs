use bitflags::bitflags;

use crate::serde_bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PublicUserFlags: u32 {
        const STAFF = 1 << 0;
        const CTP_MEMBER = 1 << 1;
        const PARTNER = 1 << 2;
        const BUG_HUNTER = 1 << 3;
        const FRIENDLY_BOT = 1 << 4;
        const FRIENDLY_BOT_MANUAL_APPROVAL = 1 << 5;
    }
}

serde_bitflags! {PublicUserFlags, u32}
