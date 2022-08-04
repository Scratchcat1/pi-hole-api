use crate::custom_deserializers;
use crate::fake_hash_map;
use crate::ftl_types::*;
use chrono::prelude::*;
use serde::Deserialize;
use serde_tuple::*;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// Summary Raw Struct
#[derive(Deserialize, Debug)]
pub struct SummaryRaw {
    /// Number of domains being blocked
    pub domains_being_blocked: u64,

    /// Number of DNS queries today
    pub dns_queries_today: u64,

    /// Number of Ads blocked today
    pub ads_blocked_today: u64,

    /// Percentage of queries blocked today
    pub ads_percentage_today: f64,

    /// Number of unique domains
    pub unique_domains: u64,

    /// Number of queries forwarded
    pub queries_forwarded: u64,

    /// Number of queries cached
    pub queries_cached: u64,

    /// Number of clients ever seen
    pub clients_ever_seen: u64,

    /// Number of unique clients
    pub unique_clients: u64,

    /// Number of DNS queries of all types
    pub dns_queries_all_types: u64,

    /// Number of NODATA replies
    #[serde(rename = "reply_NODATA")]
    pub reply_nodata: u64,

    /// Number of NXDOMAIN replies
    #[serde(rename = "reply_NXDOMAIN")]
    pub reply_nxdomain: u64,

    /// Number of CNAME replies
    #[serde(rename = "reply_CNAME")]
    pub reply_cname: u64,

    /// Number of IP replies
    #[serde(rename = "reply_IP")]
    pub reply_ip: u64,

    /// Privacy level
    pub privacy_level: u64,

    /// Pi Hole status
    pub status: String,
}

/// Summary Struct
#[derive(Deserialize, Debug)]
pub struct Summary {
    /// Formatted number of domains being blocked
    pub domains_being_blocked: String,

    /// Formatted number of DNS queries today
    pub dns_queries_today: String,

    /// Formatted number of Ads blocked today
    pub ads_blocked_today: String,

    /// Formatted percentage of queries blocked today
    pub ads_percentage_today: String,

    /// Formatted number of unique domains
    pub unique_domains: String,

    /// Formatted number of queries forwarded
    pub queries_forwarded: String,

    /// Formatted number of queries cached
    pub queries_cached: String,

    /// Formatted number of clients ever seen
    pub clients_ever_seen: String,

    /// Formatted number of unique clients
    pub unique_clients: String,

    /// Formatted number of DNS queries of all types
    pub dns_queries_all_types: String,

    /// Formatted number of NODATA replies
    #[serde(rename = "reply_NODATA")]
    pub reply_nodata: String,

    /// Formatted number of NXDOMAIN replies
    #[serde(rename = "reply_NXDOMAIN")]
    pub reply_nxdomain: String,

    /// Formatted number of CNAME replies
    #[serde(rename = "reply_CNAME")]
    pub reply_cname: String,

    /// Formatted number of IP replies
    #[serde(rename = "reply_IP")]
    pub reply_ip: String,

    /// Privacy level
    pub privacy_level: String,

    /// Pi Hole status
    pub status: String,
}

/// Over Time Data Struct
#[derive(Deserialize, Debug)]
pub struct OverTimeData {
    /// Mapping from time to number of domains
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub domains_over_time: HashMap<String, u64>,

    /// Mapping from time to number of ads
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub ads_over_time: HashMap<String, u64>,
}

/// Top Items Struct
#[derive(Deserialize, Debug)]
pub struct TopItems {
    /// Top queries mapping from domain to number of requests
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_queries: HashMap<String, u64>,

    /// Top ads mapping from domain to number of requests
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_ads: HashMap<String, u64>,
}

/// Top Clients Struct
#[derive(Deserialize, Debug)]
pub struct TopClients {
    /// Top sources mapping from "IP" or "hostname|IP" to number of requests.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_sources: HashMap<String, u64>,
}

/// Top Clients Blocked Struct
#[derive(Deserialize, Debug)]
pub struct TopClientsBlocked {
    /// Top sources blocked mapping from "IP" or "hostname|IP" to number of blocked requests.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_sources_blocked: HashMap<String, u64>,
}

/// Forward Destinations Struct
#[derive(Deserialize, Debug)]
pub struct ForwardDestinations {
    /// Forward destinations mapping from "human_readable_name|IP" to the percentage of requests answered.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub forward_destinations: HashMap<String, f64>,
}

/// Query Types Struct
#[derive(Deserialize, Debug)]
pub struct QueryTypes {
    /// Query types mapping from query type (A, AAAA, PTR, etc.) to the percentage of queries which are of that type.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub querytypes: HashMap<String, f64>,
}

/// Query Struct
#[derive(Deserialize_tuple, Debug)]
pub struct Query {
    /// Timestamp of query
    #[serde(deserialize_with = "custom_deserializers::deserialize_string_to_naive_datetime")]
    pub timestring: NaiveDateTime,

    /// Type of query (A, AAAA, PTR, etc.)
    pub query_type: QueryType,

    /// Requested domain name
    pub domain: String,

    /// Requesting client IP or hostname
    pub client: String,

    /// Status as String
    #[serde(deserialize_with = "custom_deserializers::deserialize_string_to_query_status")]
    pub status: QueryStatus,

    /// DNSSEC Status
    #[serde(deserialize_with = "custom_deserializers::deserialize_string_to_dnssec_status")]
    pub dnssec_status: DNSSECStatus,

    /// Reply Type
    #[serde(deserialize_with = "custom_deserializers::deserialize_string_to_reply_type")]
    pub reply_type: ReplyType,

    /// Response time
    #[serde(
        deserialize_with = "custom_deserializers::deserialize_string_to_duration_100_microseconds"
    )]
    pub response_time: Duration,

    /// CNAME domain
    pub cname_domain: String,

    /// Regex ID
    #[serde(deserialize_with = "custom_deserializers::deserialize_string_to_i32")]
    pub regex_id: i32,

    /// Upstream Destination
    pub upstream_destination: String,

    /// EDE
    pub ede: String,
}

/// All Queries Struct
#[derive(Deserialize, Debug)]
pub struct AllQueries {
    /// List of queries
    pub data: Vec<Query>,
}

/// Status Struct
#[derive(Deserialize, Debug)]
pub struct Status {
    /// Status, "enabled" or "disabled"
    pub status: String,
}

/// Version Struct
#[derive(Deserialize, Debug)]
pub struct Version {
    /// Version
    pub version: u32,
}

/// Versions Struct
#[derive(Deserialize, Debug)]
pub struct Versions {
    /// Is there an update available for Pi-hole core
    pub core_update: bool,
    /// Is there an update available for Pi-hole web
    pub web_update: bool,
    /// Is there an update available for Pi-hole FTL
    #[serde(rename = "FTL_update")]
    pub ftl_update: bool,
    /// Current Pi-hole core version
    pub core_current: String,
    /// Current Pi-hole web version
    pub web_current: String,
    /// Current Pi-hole FTL version
    #[serde(rename = "FTL_current")]
    pub ftl_current: String,
    /// Latest Pi-hole core version
    pub core_latest: String,
    /// Latest Pi-hole web version
    pub web_latest: String,
    /// Latest Pi-hole FTL version
    #[serde(rename = "FTL_latest")]
    pub ftl_latest: String,
    /// Current Pi-hole core branch
    pub core_branch: String,
    /// Current Pi-hole web branch
    pub web_branch: String,
    /// Current Pi-hole FTL branch
    #[serde(rename = "FTL_branch")]
    pub ftl_branch: String,
}

/// Cache Info Struct
#[derive(Deserialize, Debug)]
pub struct CacheInfo {
    /// Cache size
    #[serde(rename = "cache-size")]
    pub cache_size: u64,

    /// Number of evicted cache entries
    #[serde(rename = "cache-live-freed")]
    pub cache_live_freed: u64,

    /// Number of cache entries inserted
    #[serde(rename = "cache-inserted")]
    pub cache_inserted: u64,
}

/// Client Name Struct
#[derive(Deserialize, Debug)]
pub struct ClientName {
    /// Client name
    pub name: String,

    /// Client IP
    pub ip: IpAddr,
}

/// Network Client Struct
#[derive(Deserialize, Debug)]
pub struct NetworkClient {
    /// Client ID
    pub id: u64,

    /// IP addresses of client
    pub ip: Vec<IpAddr>,

    /// Hardware address
    pub hwaddr: String,

    /// Interface
    pub interface: String,

    /// Hostname
    pub name: Vec<String>,

    /// Time first seen
    #[serde(rename = "firstSeen")]
    pub first_seen: u64,

    /// Time of last query
    #[serde(rename = "lastQuery")]
    pub last_query: u64,

    /// Number of queries
    #[serde(rename = "numQueries")]
    pub num_queries: u64,

    /// MAC Vendor
    #[serde(rename = "macVendor")]
    pub mac_vendor: String,
}

/// Network Struct
#[derive(Deserialize, Debug)]
pub struct Network {
    /// List of network clients
    pub network: Vec<NetworkClient>,
}

/// List Modification Response Struct
#[derive(Deserialize, Debug)]
pub struct ListModificationResponse {
    /// If request was successful
    pub success: bool,
    /// Optional message about request
    pub message: Option<String>,
}

/// Custom List Domain Struct
#[derive(Deserialize, Debug)]
pub struct CustomListDomainDetails {
    /// Entry ID
    pub id: u64,
    /// Type
    #[serde(rename = "type")]
    pub domain_type: u64,
    /// Domain
    pub domain: String,
    /// Enabled
    #[serde(deserialize_with = "custom_deserializers::deserialize_uint_to_bool")]
    pub enabled: bool,
    /// Date added
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub date_added: NaiveDateTime,
    /// Date modified
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub date_modified: NaiveDateTime,
    /// Comments
    pub comment: String,
    /// Groups
    pub groups: Vec<u64>,
}

/// Local/Custom List Domain Struct
#[derive(Deserialize, Debug)]
pub struct CustomDNSRecord {
    /// Domain of record
    pub domain: String,
    /// IP Address
    pub ip_address: IpAddr,
}

/// Local/Custom List CNAME Struct
#[derive(Deserialize, Debug)]
pub struct CustomCNAMERecord {
    /// Domain of record
    pub domain: String,
    /// Target domain
    pub target_domain: String,
}

/// Response format when requesting information from the FTL while it is not running
#[derive(Deserialize, Debug)]
pub struct FTLNotRunning {
    /// Not running flag
    #[serde(rename = "FTLnotrunning")]
    pub ftl_not_running: bool,
}
