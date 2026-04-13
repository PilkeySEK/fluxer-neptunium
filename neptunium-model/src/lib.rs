//! Types and certain helpers, mainly for the Fluxer Gateway API.

pub mod channel;
pub mod gateway;
pub mod guild;
pub mod id;
pub mod invites;
pub mod misc;
pub mod time;
pub mod user;

#[macro_export]
macro_rules! serde_bitflags {
    ($name:ty, String($ty:ty)) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(Self::from_bits_truncate(
                    s.parse::<$ty>().map_err(serde::de::Error::custom)?,
                ))
            }
        }
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.bits().to_string())
            }
        }
    };
    ($name:ty, $ty:ty) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self::from_bits_truncate(<$ty>::deserialize(deserializer)?))
            }
        }
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <$ty as serde::Serialize>::serialize(&self.bits(), serializer)
            }
        }
    };
}
