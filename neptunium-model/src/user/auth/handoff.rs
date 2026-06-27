use std::{fmt::Write, str::FromStr};

use serde::{Deserialize, Serialize};

pub const HANDOFF_CODE_LENGTH: usize = 12;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct HandoffCode(pub [char; HANDOFF_CODE_LENGTH]);

impl std::fmt::Display for HandoffCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Could probably be much more optimized
        const HALF: usize = HANDOFF_CODE_LENGTH / 2;
        let first_half = &self.0[0..HALF];
        let second_half = &self.0[HALF..];
        for char in first_half {
            f.write_char(*char)?;
        }
        f.write_char('-')?;
        for char in second_half {
            f.write_char(*char)?;
        }
        Ok(())
    }
}

impl FromStr for HandoffCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const HALF: usize = HANDOFF_CODE_LENGTH / 2;

        let (first_half, second_half) = if let Some(splitted) = s.split_once('-') {
            splitted
        } else {
            let Some(splitted) = s.split_at_checked(HALF) else {
                return Err(());
            };
            splitted
        };
        if first_half.len() != HALF || second_half.len() != HALF {
            return Err(());
        }
        let mut chars = [' '; HANDOFF_CODE_LENGTH];
        let first_half = first_half.chars().enumerate();
        for (i, char) in first_half {
            chars[i] = char;
        }
        let second_half = second_half.chars().enumerate();
        for (i, char) in second_half {
            chars[i + HALF] = char;
        }
        Ok(Self(chars))
    }
}

impl Serialize for HandoffCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HandoffCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("failed to parse handoff code"))
    }
}
