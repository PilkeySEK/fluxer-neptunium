use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::IdMarker};

/// An ID in the form of `left:right`, where `left` and `right` are IDs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CompositeId<L: IdMarker, R: IdMarker> {
    pub left: Id<L>,
    pub right: Id<R>,
}

impl<L: IdMarker, R: IdMarker> CompositeId<L, R> {
    #[must_use]
    pub fn new(left: Id<L>, right: Id<R>) -> Self {
        Self { left, right }
    }

    #[must_use]
    pub fn get_left(&self) -> &Id<L> {
        &self.left
    }

    #[must_use]
    pub fn get_right(&self) -> &Id<R> {
        &self.right
    }
}

impl<L: IdMarker, R: IdMarker> From<CompositeId<L, R>> for (Id<L>, Id<R>) {
    fn from(value: CompositeId<L, R>) -> Self {
        (value.left, value.right)
    }
}

impl<L: IdMarker, R: IdMarker> From<(Id<L>, Id<R>)> for CompositeId<L, R> {
    fn from(value: (Id<L>, Id<R>)) -> Self {
        Self {
            left: value.0,
            right: value.1,
        }
    }
}

impl<'de, L: IdMarker, R: IdMarker> Deserialize<'de> for CompositeId<L, R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        let Some((left, right)) = string.split_once(':') else {
            return Err(serde::de::Error::custom("invalid composite id"));
        };

        let left: u64 = left.parse().map_err(|e| {
            serde::de::Error::custom(format!("failed to parse left composite ID: {e}"))
        })?;
        let right: u64 = right.parse().map_err(|e| {
            serde::de::Error::custom(format!("failed to parse right composite ID: {e}"))
        })?;

        Ok(Self {
            left: Id::<L>::from(left),
            right: Id::<R>::from(right),
        })
    }
}

impl<L: IdMarker, R: IdMarker> Serialize for CompositeId<L, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Maybe use a more efficient way of creating this string
        let s = format!("{}:{}", self.left, self.right);
        s.serialize(serializer)
    }
}
