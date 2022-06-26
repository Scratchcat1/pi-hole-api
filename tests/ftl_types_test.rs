use pi_hole_api::ftl_types::*;
use strum::{EnumCount, IntoEnumIterator};

#[test]
fn serialise_dnssec_status_test() {
    for (i, status) in DNSSECStatus::iter().enumerate() {
        assert_eq!(serde_json::to_string(&status).unwrap(), i.to_string());
    }
}

#[test]
fn deserialise_dnssec_status_test() {
    for (i, status) in DNSSECStatus::iter().enumerate() {
        assert_eq!(
            serde_json::from_str::<DNSSECStatus>(&i.to_string()).unwrap(),
            status
        );
    }
}

#[test]
#[should_panic]
fn panic_deserialise_invalid_dnssec_status_value_test() {
    serde_json::from_str::<DNSSECStatus>(&(DNSSECStatus::COUNT + 1).to_string()).unwrap();
}

#[test]
fn serialise_query_status_test() {
    for (i, status) in QueryStatus::iter().enumerate() {
        assert_eq!(serde_json::to_string(&status).unwrap(), i.to_string());
    }
}

#[test]
fn deserialise_query_status_test() {
    for (i, status) in QueryStatus::iter().enumerate() {
        assert_eq!(
            serde_json::from_str::<QueryStatus>(&i.to_string()).unwrap(),
            status
        );
    }
}

#[test]
#[should_panic]
fn panic_deserialise_invalid_query_status_value_test() {
    serde_json::from_str::<QueryStatus>(&(QueryStatus::COUNT + 1).to_string()).unwrap();
}
