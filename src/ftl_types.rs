use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use strum::{EnumCount, EnumIter};

// These types are taken from enums.h in the Pi-Hole FTL repository
// and modified to be able to convert the integer values into something more easily usable

#[derive(
    Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug, EnumCount, EnumIter, FromPrimitive,
)]
#[repr(u8)]
pub enum DNSSECStatus {
    DNSSECUnspecified,
    DNSSECSecure,
    DNSSECInsecure,
    DNSSECBogus,
    DNSSECAbandoned,
}

#[derive(
    Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug, EnumCount, EnumIter, FromPrimitive,
)]
#[repr(u8)]
pub enum QueryStatus {
    QueryUnknown,
    QueryGravity,
    QueryForwarded,
    QueryCache,
    QueryRegex,
    QueryBlacklist,
    QueryExternalBlockedIp,
    QueryExternalBlockedNull,
    QueryExternalBlockedNxra,
    QueryGravityCname,
    QueryRegexCname,
    QueryBlacklistCname,
    QueryRetried,
    QueryRetriedDnssec,
    QueryInProgress,
    QueryDbbusy,
    QueryStatusMax,
}

#[derive(
    Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug, EnumCount, EnumIter, FromPrimitive,
)]
#[repr(u8)]
pub enum ReplyType {
    ReplyUNKNOWN,
    ReplyNODATA,
    ReplyNXDOMAIN,
    ReplyCNAME,
    ReplyIP,
    ReplyDOMAIN,
    ReplyRRNAME,
    ReplySERVFAIL,
    ReplyREFUSED,
    ReplyNOTIMP,
    ReplyOTHER,
    ReplyDNSSEC,
    ReplyNONE,
    ReplyBLOB,
    QueryReplyMax,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, EnumCount, EnumIter, FromPrimitive)]
pub enum QueryType {
    A,
    AAAA,
    ANY,
    SRV,
    SOA,
    PTR,
    TXT,
    NAPTR,
    MX,
    DS,
    RRSIG,
    DNSKEY,
    NS,
    OTHER,
    SVCB,
    HTTPS,
    MAX,
}
