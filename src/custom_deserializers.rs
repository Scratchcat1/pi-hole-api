use crate::ftl_types::*;
use chrono::NaiveDateTime;
use num_traits::FromPrimitive;
use serde::{de::Deserializer, Deserialize};
use std::time::Duration;

/// Deserialize to integer and then convert into a boolean
/// 0 is false, everything else is true
pub fn deserialize_uint_to_bool<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let result = u64::deserialize(deserializer)?;
    Ok(result == 0)
}

/// Deserialize to string and then convert into an i32
/// 0 is false, everything else is true
pub fn deserialize_string_to_i32<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<i32, D::Error> {
    let result = String::deserialize(deserializer)?;
    Ok(result.parse::<i32>().unwrap())
}

/// Deserialize to a string first (format is "1656247185")
/// to get the actual string value. Then parse the string value to an i64
/// and finally create the NaiveDateTime
pub fn deserialize_string_to_naive_datetime<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error> {
    let result = String::deserialize(deserializer)?;
    Ok(NaiveDateTime::from_timestamp(
        result.parse::<i64>().unwrap(),
        0,
    ))
}

/// Deserialize to a string, then convert to a u8 and finally to a DNSSECStatus
/// e.g. "0" -> 0 -> DNSSECStatus::DNSSECUnspecified
pub fn deserialize_string_to_dnssec_status<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<DNSSECStatus, D::Error> {
    let string = String::deserialize(deserializer)?;
    let u8_value = string.parse::<u8>().unwrap();
    Ok(FromPrimitive::from_u8(u8_value).unwrap())
}

/// Deserialize to a string, then convert to a u8 and finally to a QueryStatus
/// e.g. "0" -> 0 -> QueryStatus::QueryUnknown
pub fn deserialize_string_to_query_status<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<QueryStatus, D::Error> {
    let string = String::deserialize(deserializer)?;
    let u8_value = string.parse::<u8>().unwrap();
    Ok(FromPrimitive::from_u8(u8_value).unwrap())
}

/// Deserialize to a string, then convert to a u64 of unit 100us and finally to a Duration
/// e.g. "10" -> 10 -> 1000us -> Duration(1000us)
pub fn deserialize_string_to_duration_100_microseconds<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Duration, D::Error> {
    let string = String::deserialize(deserializer)?;
    let duration_in_100us = string.parse::<u64>().unwrap();
    Ok(Duration::from_micros(100 * duration_in_100us))
}

// /// Deserialize to a string, then convert to a u8 and finally to a QueryType
// /// e.g. "0" -> 0 -> QueryType::TypeA
// pub fn deserialize_string_to_query_type<'de, D: Deserializer<'de>>(
//     deserializer: D,
// ) -> Result<QueryType, D::Error> {
//     let string = String::deserialize(deserializer)?;
//     let u8_value = string.parse::<u8>().unwrap();
//     Ok(FromPrimitive::from_u8(u8_value).unwrap())
// }

/// Deserialize to a string, then convert to a u8 and finally to a ReplyType
/// e.g. "0" -> 0 -> ReplyType::ReplyUNKNOWN
pub fn deserialize_string_to_reply_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ReplyType, D::Error> {
    let string = String::deserialize(deserializer)?;
    let u8_value = string.parse::<u8>().unwrap();
    Ok(FromPrimitive::from_u8(u8_value).unwrap())
}
