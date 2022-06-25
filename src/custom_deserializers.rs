use serde::{de::Deserializer, Deserialize};

/// Deserialise to integer and then convert into a boolean
/// 0 is false, everything else is true
pub fn deserialize_uint_to_bool<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let result = u64::deserialize(deserializer)?;
    Ok(result == 0)
}
