use pi_hole_api::ftl_types::*;
use strum::{EnumCount, IntoEnumIterator};

#[test]
fn serialize_dnssec_status_test() {
    for (i, status) in DNSSECStatus::iter().enumerate() {
        assert_eq!(serde_json::to_string(&status).unwrap(), i.to_string());
    }
}

#[test]
fn deserialize_dnssec_status_test() {
    for (i, status) in DNSSECStatus::iter().enumerate() {
        assert_eq!(
            serde_json::from_str::<DNSSECStatus>(&i.to_string()).unwrap(),
            status
        );
    }
}

#[test]
#[should_panic]
fn panic_deserialize_invalid_dnssec_status_value_test() {
    serde_json::from_str::<DNSSECStatus>(&(DNSSECStatus::COUNT + 1).to_string()).unwrap();
}

#[test]
fn serialize_query_status_test() {
    for (i, status) in QueryStatus::iter().enumerate() {
        assert_eq!(serde_json::to_string(&status).unwrap(), i.to_string());
    }
}

#[test]
fn deserialize_query_status_test() {
    for (i, status) in QueryStatus::iter().enumerate() {
        assert_eq!(
            serde_json::from_str::<QueryStatus>(&i.to_string()).unwrap(),
            status
        );
    }
}

#[test]
#[should_panic]
fn panic_deserialize_invalid_query_status_value_test() {
    serde_json::from_str::<QueryStatus>(&(QueryStatus::COUNT + 1).to_string()).unwrap();
}

#[test]
fn serialize_reply_type_test() {
    for (i, status) in ReplyType::iter().enumerate() {
        assert_eq!(serde_json::to_string(&status).unwrap(), i.to_string());
    }
}

#[test]
fn deserialize_reply_type_test() {
    for (i, status) in ReplyType::iter().enumerate() {
        assert_eq!(
            serde_json::from_str::<ReplyType>(&i.to_string()).unwrap(),
            status
        );
    }
}

#[test]
#[should_panic]
fn panic_deserialize_invalid_reply_type_value_test() {
    serde_json::from_str::<ReplyType>(&(ReplyType::COUNT + 1).to_string()).unwrap();
}
