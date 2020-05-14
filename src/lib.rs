use reqwest::{self, Error};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::IpAddr;

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
    pub domains_over_time: HashMap<i64, u64>,

    /// Mapping from time to number of ads
    pub ads_over_time: HashMap<i64, u64>,
}

/// Top Items Struct
#[derive(Deserialize, Debug)]
pub struct TopItems {
    /// Top queries
    pub top_queries: HashMap<String, u64>,

    /// Top ads
    pub top_ads: HashMap<String, u64>,
}

/// Top Clients Struct
#[derive(Deserialize, Debug)]
pub struct TopClients {
    /// Top sources
    pub top_sources: HashMap<String, u64>,
}

/// Top Clients Blocked Struct
#[derive(Deserialize, Debug)]
pub struct TopClientsBlocked {
    /// Top sources blocked
    pub top_sources_blocked: HashMap<String, u64>,
}

/// Forward Destinations Struct
#[derive(Deserialize, Debug)]
pub struct ForwardDestinations {
    /// Forward destinations
    pub forward_destinations: HashMap<String, f64>,
}

/// Query Types Struct
#[derive(Deserialize, Debug)]
pub struct QueryTypes {
    /// Query types
    pub querytypes: HashMap<String, f64>,
}

/// Query Struct
#[derive(Deserialize, Debug)]
pub struct Query {
    /// Timestamp of query
    pub timestring: String,

    /// Type of query
    pub query_type: String,

    /// Requested domain name
    pub domain: String,

    /// Requesting client
    pub client: String,

    /// Status as String
    pub answer_type: String,
}

/// All Queries Struct
#[derive(Deserialize, Debug)]
pub struct AllQueries {
    /// List of queries
    data: Vec<Query>,
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

/// Pi Hole API Struct
pub struct PiHoleAPI {
    /// Pi Hole host
    host: String,

    /// Optional API key
    api_key: Option<String>,
}

impl PiHoleAPI {
    /// Creates a new Pi Hole API instance
    pub fn new(host: String, api_key: Option<String>) -> Self {
        Self { host, api_key }
    }

    pub fn set_api_key(&mut self, api_key: &String) {
        self.api_key = Some(api_key.into());
    }

    /// Get statistics in a raw format (no number format)
    pub async fn get_summary_raw(&self) -> Result<SummaryRaw, Error> {
        let url = format!("{}/admin/api.php?summaryRaw", self.host);
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get statistics in a formatted style
    pub async fn get_summary(&self) -> Result<Summary, Error> {
        let url = format!("{}/admin/api.php?summary", self.host);
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get statistics on the number of domains and ads for each 10 minute period
    pub async fn get_over_time_data_10_mins(&self) -> Result<OverTimeData, Error> {
        let url = format!("{}/admin/api.php?overTimeData10mins", self.host);
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get the top domains and ads and the number of queries for each. Limit the number of items with `count`.
    /// API key required.
    pub async fn get_top_items(&self, count: Option<u32>) -> Result<TopItems, Error> {
        let url = format!(
            "{}/admin/api.php?topItems={}&auth={}",
            self.host,
            count.unwrap_or(10),
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get the top clients and the number of queries for each. Limit the number of items with `count`.
    /// API key required.
    pub async fn get_top_clients(&self, count: Option<u32>) -> Result<TopClients, Error> {
        let url = format!(
            "{}/admin/api.php?topClients={}&auth={}",
            self.host,
            count.unwrap_or(10),
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get the top clients blocked and the number of queries for each. Limit the number of items with `count`.
    /// API key required.
    pub async fn get_top_clients_blocked(
        &self,
        count: Option<u32>,
    ) -> Result<TopClientsBlocked, Error> {
        let url = format!(
            "{}/admin/api.php?topClientsBlocked={}&auth={}",
            self.host,
            count.unwrap_or(10),
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get the number of queries forwarded and the target.
    /// API key required.
    pub async fn get_forward_destinations(&self) -> Result<ForwardDestinations, Error> {
        let url = format!(
            "{}/admin/api.php?getForwardDestinations&auth={}",
            self.host,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get the number of queries per type.
    /// API key required.
    pub async fn get_query_types(&self) -> Result<QueryTypes, Error> {
        let url = format!(
            "{}/admin/api.php?getQueryTypes&auth={}",
            self.host,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get all DNS query data. Limit the number of items with `count`.
    /// API key required.
    pub async fn get_all_queries(&self, count: u32) -> Result<AllQueries, Error> {
        let url = format!(
            "{}/admin/api.php?getAllQueries={}&auth={}",
            self.host,
            count,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        let mut raw_data: HashMap<String, Vec<Vec<String>>> = response.json().await?;
        let data = AllQueries {
            data: raw_data
                .remove("data")
                .unwrap()
                .iter()
                .map(|raw_query| Query {
                    timestring: raw_query[0].clone(),
                    query_type: raw_query[1].clone(),
                    domain: raw_query[2].clone(),
                    client: raw_query[3].clone(),
                    answer_type: raw_query[4].clone(),
                })
                .collect(),
        };
        Ok(data)
    }

    /// Enable the Pi-Hole.
    /// API key required.
    pub async fn enable(&self) -> Result<Status, Error> {
        let url = format!(
            "{}/admin/api.php?enable&auth={}",
            self.host,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Disable the Pi-Hole for `seconds` seconds.
    /// API key required.
    pub async fn disable(&self, seconds: u64) -> Result<Status, Error> {
        let url = format!(
            "{}/admin/api.php?disable={}&auth={}",
            self.host,
            seconds,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get the Pi-Hole version.
    pub async fn get_version(&self) -> Result<Version, Error> {
        let url = format!("{}/admin/api.php?version", self.host);
        let response = reqwest::get(&url).await?;
        Ok(response.json().await?)
    }

    /// Get statistics about the DNS cache.
    /// API key required.
    pub async fn get_cache_info(&self) -> Result<CacheInfo, Error> {
        let url = format!(
            "{}/admin/api.php?getCacheInfo&auth={}",
            self.host,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        let mut raw_data: HashMap<String, CacheInfo> = response.json().await?;
        Ok(raw_data.remove("cacheinfo").expect("Missing cache info"))
    }

    /// Get hostname and IP for hosts
    /// API key required.
    pub async fn get_client_names(&self) -> Result<Vec<ClientName>, Error> {
        let url = format!(
            "{}/admin/api.php?getClientNames&auth={}",
            self.host,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        let mut raw_data: HashMap<String, Vec<ClientName>> = response.json().await?;
        Ok(raw_data
            .remove("clients")
            .expect("Missing clients attribute"))
    }

    /// Get queries by client over time. Maps timestamp to the number of queries by clients.
    /// Order of clients in the Vector is the same as for get_client_names
    /// API key required.
    pub async fn get_over_time_data_clients(&self) -> Result<HashMap<u64, Vec<u64>>, Error> {
        let url = format!(
            "{}/admin/api.php?overTimeDataClients&auth={}",
            self.host,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );
        let response = reqwest::get(&url).await?;
        let mut raw_data: HashMap<String, HashMap<u64, Vec<u64>>> = response.json().await?;
        Ok(raw_data
            .remove("over_time")
            .expect("Missing over_time attribute"))
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use tokio;

    fn pi_hole_api_test_target() -> String {
        env::var("PI_HOLE_API_TEST_TARGET")
            .expect("Missing environmental var PI_HOLE_API_TEST_TARGET")
    }

    fn pi_hole_api_test_api_key() -> Option<String> {
        Some(
            env::var("PI_HOLE_API_TEST_API_KEY")
                .expect("Missing environmental var PI_HOLE_API_TEST_API_KEY"),
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn get_summary_raw_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), None);
        match api.get_summary_raw().await {
            Ok(summary_raw) => {
                println!("{:?}", summary_raw.status);
                assert!(summary_raw.status == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get summary raw: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_summary_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), None);
        match api.get_summary().await {
            Ok(summary) => {
                println!("{:?}", summary.status);
                assert!(summary.status == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get summary: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_over_time_data_10_mins_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), None);
        match api.get_over_time_data_10_mins().await {
            Ok(over_time_data) => {
                println!("{:?}", over_time_data);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(
                false,
                format!("Failed to get over time data 10 minutes: {}", e)
            ),
        };
    }

    #[tokio::test]
    async fn get_top_items_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_top_items(None).await {
            Ok(top_items) => {
                println!("{:?}", top_items);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };

        match api.get_top_items(Some(1)).await {
            Ok(top_items) => {
                println!("{:?}", top_items);
                assert!(top_items.top_queries.len() <= 1);
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };

        match api.get_top_items(Some(100)).await {
            Ok(top_items) => {
                println!("{:?}", top_items);
                assert!(top_items.top_queries.len() <= 100);
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_top_clients_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_top_clients(None).await {
            Ok(top_clients) => {
                println!("{:?}", top_clients);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };

        match api.get_top_clients(Some(1)).await {
            Ok(top_clients) => {
                println!("{:?}", top_clients);
                assert!(top_clients.top_sources.len() <= 1);
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };

        match api.get_top_clients(Some(100)).await {
            Ok(top_clients) => {
                println!("{:?}", top_clients);
                assert!(top_clients.top_sources.len() <= 100);
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_top_clients_blocked_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_top_clients_blocked(None).await {
            Ok(top_clients_blocked) => {
                println!("{:?}", top_clients_blocked);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };

        match api.get_top_clients_blocked(Some(1)).await {
            Ok(top_clients_blocked) => {
                println!("{:?}", top_clients_blocked);
                assert!(top_clients_blocked.top_sources_blocked.len() <= 1);
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };

        match api.get_top_clients_blocked(Some(100)).await {
            Ok(top_clients_blocked) => {
                println!("{:?}", top_clients_blocked);
                assert!(top_clients_blocked.top_sources_blocked.len() <= 100);
            }
            Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_forward_destinations_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_forward_destinations().await {
            Ok(forward_destinations) => {
                println!("{:?}", forward_destinations);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get forward destinations: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_query_types_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_query_types().await {
            Ok(query_types) => {
                println!("{:?}", query_types);
                assert!(query_types.querytypes.get("A (IPv4)").expect("Missing key") >= &0.0);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get query types: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_all_queries_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_all_queries(100).await {
            Ok(all_queries) => {
                println!("{:?}", all_queries);
                // assert!(all_queries.data.len() >= 0);
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to get all queries: {}", e)),
        };
    }

    #[tokio::test]
    async fn enable_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.enable().await {
            Ok(status) => {
                println!("{:?}", status);
                assert!(status.status == "enabled");
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to enable pi-hole: {}", e)),
        };
    }

    #[tokio::test]
    async fn disable_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.disable(10).await {
            Ok(status) => {
                println!("{:?}", status);
                assert!(status.status == "disabled");
                // assert!(over_time_data == "enabled")
            }
            Err(e) => assert!(false, format!("Failed to disable pi-hole: {}", e)),
        };
        api.enable()
            .await
            .expect("Failed to reenable pi-hole after test");
    }

    #[tokio::test]
    async fn version_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), None);
        match api.get_version().await {
            Ok(version) => {
                println!("{:?}", version);
                assert!(version.version >= 3);
            }
            Err(e) => assert!(false, format!("Failed to get version: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_cache_info_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_cache_info().await {
            Ok(cache_info) => {
                println!("{:?}", cache_info);
                // assert!(cache_info.cache >= 3);
            }
            Err(e) => assert!(false, format!("Failed to get cache info: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_client_names_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_client_names().await {
            Ok(client_names) => {
                println!("{:?}", client_names);
                assert!(client_names.len() > 0);
                // assert!(cache_info.cache >= 3);
            }
            Err(e) => assert!(false, format!("Failed to get client names: {}", e)),
        };
    }

    #[tokio::test]
    async fn get_over_time_data_clients_test() {
        let api = crate::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
        match api.get_over_time_data_clients().await {
            Ok(over_time_data_clients) => {
                println!("{:?}", over_time_data_clients);
                assert!(over_time_data_clients.len() > 0);
                // assert!(cache_info.cache >= 3);
            }
            Err(e) => assert!(
                false,
                format!("Failed to get over time data clients: {}", e)
            ),
        };
    }
}
