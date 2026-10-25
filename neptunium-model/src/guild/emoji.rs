use std::fmt::Write;

use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::EmojiMarker};

#[derive(Deserialize, Serialize)]
struct RawEmoji {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<EmojiMarker>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub animated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Emoji {
    /// A default emoji (not guild-specific). The value is a unicode emoji.
    Default(String),
    /// A custom emoji from a guild.
    Custom {
        name: String,
        id: Id<EmojiMarker>,
        animated: bool,
    },
}

impl Emoji {
    /// Parse this emoji from a unicode emoji or custom emoji string.
    ///
    /// Returns `None` if parsing failed.
    ///
    /// # Examples
    /// ```
    /// # use neptunium_model::{guild::Emoji, id::Id};
    /// # fn main() {
    /// let default_emoji = "🪑";
    /// let custom_emoji = "<a:happyghost:1431008660295635538>";
    ///
    /// assert_eq!(
    ///     Emoji::parse(default_emoji),
    ///     Some(Emoji::Default(default_emoji.to_string()))
    /// );
    /// assert_eq!(
    ///     Emoji::parse(custom_emoji),
    ///     Some(Emoji::Custom {
    ///         name: "happyghost".to_string(),
    ///         id: Id::new(1431008660295635538),
    ///         animated: true,
    ///     })
    /// );
    /// # }
    /// ```
    #[must_use]
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if let Some(input) = input.strip_prefix('<') {
            let input = input.strip_suffix('>')?;
            let mut parts = input.split(':');
            let first_part = parts.next()?;
            if first_part == "a" {
                let name = parts.next()?;
                let id = Id::try_from(parts.next()?).ok()?;
                if parts.next().is_some() {
                    None
                } else {
                    Some(Self::Custom {
                        name: name.to_owned(),
                        id,
                        animated: true,
                    })
                }
            } else if first_part.is_empty() {
                let name = parts.next()?;
                let id = Id::try_from(parts.next()?).ok()?;
                if parts.next().is_some() {
                    None
                } else {
                    Some(Self::Custom {
                        name: name.to_owned(),
                        id,
                        animated: false,
                    })
                }
            } else {
                None
            }
        } else {
            Some(Self::Default(input.to_owned()))
        }
    }
}

impl<'de> Deserialize<'de> for Emoji {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_emoji = RawEmoji::deserialize(deserializer)?;

        Ok(if let Some(id) = raw_emoji.id {
            Self::Custom {
                name: raw_emoji.name,
                id,
                animated: raw_emoji.animated,
            }
        } else {
            Self::Default(raw_emoji.name)
        })
    }
}

impl Serialize for Emoji {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let raw_emoji = match self.clone() {
            Self::Custom { name, id, animated } => RawEmoji {
                name,
                id: Some(id),
                animated,
            },
            Self::Default(name) => RawEmoji {
                name,
                id: None,
                animated: false,
            },
        };
        raw_emoji.serialize(serializer)
    }
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom { name, id, animated } => {
                f.write_char('<')?;
                if *animated {
                    f.write_char('a')?;
                }
                f.write_char(':')?;
                name.fmt(f)?;
                f.write_char(':')?;
                id.fmt(f)?;
                f.write_char('>')?;
            }
            Self::Default(name) => {
                name.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl From<&str> for Emoji {
    /// The value should be a unicode emoji.
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<String> for Emoji {
    /// The value should be a unicode emoji.
    fn from(value: String) -> Self {
        Self::Default(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(clippy::unreadable_literal)]
    fn emojis() {
        let default_emoji = "🪑";
        let custom_emoji_1 = "<a:abc:1484905952721252427>";
        let custom_emoji_2 = "<:abc:1484905952721252427>";
        let invalid_custom_emoji_1 = "<:1484905952721252427>";
        let invalid_custom_emoji_2 = "<b:abc:1484905952721252427>";

        assert_eq!(
            Emoji::parse(default_emoji),
            Some(Emoji::Default(default_emoji.to_string()))
        );
        assert_eq!(
            Emoji::parse(custom_emoji_1),
            Some(Emoji::Custom {
                name: "abc".to_string(),
                id: Id::from(1484905952721252427),
                animated: true
            })
        );
        assert_eq!(
            Emoji::parse(custom_emoji_2),
            Some(Emoji::Custom {
                name: "abc".to_string(),
                id: Id::from(1484905952721252427),
                animated: false
            })
        );
        assert_eq!(Emoji::parse(invalid_custom_emoji_1), None);
        assert_eq!(Emoji::parse(invalid_custom_emoji_2), None);
    }
}
