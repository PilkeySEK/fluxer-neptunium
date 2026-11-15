pub mod zero_is_option_usize {
    use std::num::NonZeroUsize;

    use serde::{Deserialize, Serialize};

    #[expect(clippy::ref_option, clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: serde::Serializer>(
        value: &Option<NonZeroUsize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(num) => num.serialize(serializer),
            None => 0.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NonZeroUsize>, D::Error> {
        let num = usize::deserialize(deserializer)?;
        Ok(NonZeroUsize::new(num))
    }
}

pub mod zero_is_option_duration_seconds {
    use std::time::Duration;

    use serde::{Deserialize, Serialize};

    use crate::time::duration::Seconds;

    #[expect(clippy::ref_option)]
    pub fn serialize<S: serde::Serializer>(
        value: &Option<crate::time::duration::Duration<Seconds>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(num) => num.serialize(serializer),
            None => 0.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<crate::time::duration::Duration<Seconds>>, D::Error> {
        let duration = crate::time::duration::Duration::<Seconds>::deserialize(deserializer)?;
        Ok(
            if duration == crate::time::duration::Duration::<Seconds>::from(Duration::ZERO) {
                None
            } else {
                Some(duration)
            },
        )
    }
}
