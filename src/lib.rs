use crate::fake_hash_map::FakeHashMap;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::net::IpAddr;
pub mod api_types;
mod custom_deserializers;
pub mod errors;
mod fake_hash_map;
pub mod ftl_types;
use crate::api_types::*;
use std::borrow::Borrow;

const NO_PARAMS: [(&str, &str); 0] = [];

trait PiHoleAPIHost {
    fn get_host(&self) -> &str;
}

trait PiHoleAPIKey {
    fn get_api_key(&self) -> &str;
}

/// Pi Hole API Struct
#[derive(Debug)]
pub struct PiHoleAPIConfig {
    /// Pi Hole host
    host: String,
}

impl PiHoleAPIConfig {
    /// Creates a new Pi Hole API instance.
    /// `host` must begin with the protocol e.g. http:// or https://
    pub fn new(host: String) -> Self {
        Self { host }
    }
}

/// Pi Hole API Struct
#[derive(Debug)]
pub struct PiHoleAPIConfigWithKey {
    /// Pi Hole host
    host: String,

    /// API key
    api_key: String,
}

impl PiHoleAPIConfigWithKey {
    /// Creates a new Pi Hole API instance.
    /// `host` must begin with the protocol e.g. http:// or https://
    pub fn new(host: String, api_key: String) -> Self {
        Self { host, api_key }
    }
}

impl PiHoleAPIHost for PiHoleAPIConfig {
    fn get_host(&self) -> &str {
        &self.host
    }
}

impl PiHoleAPIHost for PiHoleAPIConfigWithKey {
    fn get_host(&self) -> &str {
        &self.host
    }
}

impl PiHoleAPIKey for PiHoleAPIConfigWithKey {
    fn get_api_key(&self) -> &str {
        &self.api_key
    }
}

pub trait UnauthenticatedPiHoleAPI {
    /// Get statistics in a raw format (no number format)
    fn get_summary_raw(&self) -> Result<SummaryRaw, errors::APIError>;

    /// Get statistics in a formatted style
    fn get_summary(&self) -> Result<Summary, errors::APIError>;

    /// Get statistics on the number of domains and ads for each 10 minute period
    fn get_over_time_data_10_mins(&self) -> Result<OverTimeData, errors::APIError>;

    /// Get the Pi-Hole version.
    fn get_version(&self) -> Result<u32, errors::APIError>;

    /// Get the detailed Pi-Hole versions for core, FTL and web interface.
    fn get_versions(&self) -> Result<Versions, errors::APIError>;
}

fn simple_json_request<T, I, K, V>(
    host: &str,
    path_query: &str,
    params: I,
) -> Result<T, errors::APIError>
where
    T: DeserializeOwned,
    I: IntoIterator,
    K: AsRef<str>,
    V: AsRef<str>,
    <I as IntoIterator>::Item: Borrow<(K, V)>,
{
    let path = format!("{}{}", host, path_query);
    let response = reqwest::blocking::get(
        reqwest::Url::parse_with_params(&path, params).expect("Invalid URL"),
    )?;
    Ok(response.json()?)
}

impl<T> UnauthenticatedPiHoleAPI for T
where
    T: PiHoleAPIHost,
{
    fn get_summary_raw(&self) -> Result<SummaryRaw, errors::APIError> {
        simple_json_request(self.get_host(), "/admin/api.php?summaryRaw", &NO_PARAMS)
    }

    fn get_summary(&self) -> Result<Summary, errors::APIError> {
        simple_json_request(self.get_host(), "/admin/api.php?summary", &NO_PARAMS)
    }

    fn get_over_time_data_10_mins(&self) -> Result<OverTimeData, errors::APIError> {
        simple_json_request(
            self.get_host(),
            "/admin/api.php?overTimeData10mins",
            &NO_PARAMS,
        )
    }

    /// Get simple PiHole version
    fn get_version(&self) -> Result<u32, errors::APIError> {
        let raw_version: Version =
            simple_json_request(self.get_host(), "/admin/api.php?version", &NO_PARAMS)?;
        Ok(raw_version.version)
    }

    /// Get versions of core, FTL and web and if updates are available
    fn get_versions(&self) -> Result<Versions, errors::APIError> {
        simple_json_request(self.get_host(), "/admin/api.php?versions", &NO_PARAMS)
    }
}

pub trait AuthenticatedPiHoleAPI {
    /// Get the top domains and ads and the number of queries for each. Limit the number of items with `count`.
    fn get_top_items(&self, count: &Option<u32>) -> Result<TopItems, errors::APIError>;

    /// Get the top clients and the number of queries for each. Limit the number of items with `count`.
    fn get_top_clients(&self, count: &Option<u32>) -> Result<TopClients, errors::APIError>;

    /// Get the top clients blocked and the number of queries for each. Limit the number of items with `count`.
    fn get_top_clients_blocked(
        &self,
        count: Option<u32>,
    ) -> Result<TopClientsBlocked, errors::APIError>;

    /// Get the percentage of queries forwarded to each target.
    fn get_forward_destinations(
        &self,
        unsorted: bool,
    ) -> Result<ForwardDestinations, errors::APIError>;

    /// Get the number of queries per type.
    fn get_query_types(&self) -> Result<QueryTypes, errors::APIError>;

    /// Get all DNS query data. Limit the number of items with `count`.
    fn get_all_queries(&self, count: u32) -> Result<Vec<Query>, errors::APIError>;

    /// Enable the Pi-Hole.
    fn enable(&self) -> Result<Status, errors::APIError>;

    /// Disable the Pi-Hole for `seconds` seconds.
    fn disable(&self, seconds: u64) -> Result<Status, errors::APIError>;

    /// Get statistics about the DNS cache.
    fn get_cache_info(&self) -> Result<CacheInfo, errors::APIError>;

    /// Get hostname and IP for hosts
    fn get_client_names(&self) -> Result<Vec<ClientName>, errors::APIError>;

    /// Get queries by client over time. Maps timestamp to the number of queries by clients.
    /// Order of clients in the Vector is the same as for get_client_names
    fn get_over_time_data_clients(&self) -> Result<HashMap<String, Vec<u64>>, errors::APIError>;

    /// Get information about network clients.
    fn get_network(&self) -> Result<Network, errors::APIError>;

    /// Get the total number of queries received.
    fn get_queries_count(&self) -> Result<u64, errors::APIError>;

    /// Add domains to a custom white/blacklist.
    /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
    fn list_add(
        &self,
        domain: &str,
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Remove domain to a custom white/blacklist.
    /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
    fn list_remove(
        &self,
        domain: &str,
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Get a list of domains on a particular custom white/blacklist
    /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
    fn list_get_domains(
        &self,
        list: &str,
    ) -> Result<Vec<CustomListDomainDetails>, errors::APIError>;

    /// Get a list of custom DNS records
    fn get_custom_dns_records(&self) -> Result<Vec<CustomDNSRecord>, errors::APIError>;

    /// Add a custom DNS record
    fn add_custom_dns_record(
        &self,
        ip: &IpAddr,
        domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Delete a custom DNS record
    fn delete_custom_dns_record(
        &self,
        ip: &IpAddr,
        domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Get a list of custom CNAME records
    fn get_custom_cname_records(&self) -> Result<Vec<CustomCNAMERecord>, errors::APIError>;

    /// Add a custom CNAME record
    fn add_custom_cname_record(
        &self,
        domain: &str,
        target_domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Delete a custom CNAME record
    fn delete_custom_cname_record(
        &self,
        domain: &str,
        target_domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Get max logage
    fn get_max_logage(&self) -> Result<f32, errors::APIError>;
}

fn authenticated_json_request<'a, T, I, K, V>(
    host: &str,
    path_query: &str,
    params: I,
    api_key: &'a str,
) -> Result<T, errors::APIError>
where
    T: DeserializeOwned,
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
    // <I as IntoIterator>::Item: Borrow<(K, V)>,
{
    let path = format!("{}{}", host, path_query);
    let auth_params = [("auth".to_string(), api_key.to_string())];
    let converted_params: Vec<(String, String)> = params
        .into_iter()
        .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
        .collect();
    let url =
        reqwest::Url::parse_with_params(&path, converted_params.iter().chain(auth_params.iter()))
            .expect("Invalid URL");
    let response_text = reqwest::blocking::get(url)?.text()?;
    errors::detect_response_errors(&response_text)?;
    match serde_json::from_str::<T>(&response_text) {
        Ok(response) => Ok(response),
        Err(error) => Err(error.into()),
    }
}

impl<T> AuthenticatedPiHoleAPI for T
where
    T: PiHoleAPIHost + PiHoleAPIKey,
{
    fn get_top_items(&self, count: &Option<u32>) -> Result<TopItems, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("topItems", count.unwrap_or(10).to_string())],
            self.get_api_key(),
        )
    }

    fn get_top_clients(&self, count: &Option<u32>) -> Result<TopClients, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php?",
            [("topClients", count.unwrap_or(10).to_string())],
            self.get_api_key(),
        )
    }

    fn get_top_clients_blocked(
        &self,
        count: Option<u32>,
    ) -> Result<TopClientsBlocked, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php?",
            [("topClientsBlocked", count.unwrap_or(10).to_string())],
            self.get_api_key(),
        )
    }

    fn get_forward_destinations(
        &self,
        unsorted: bool,
    ) -> Result<ForwardDestinations, errors::APIError> {
        let param_value = if unsorted { "unsorted" } else { "" };
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("getForwardDestinations", param_value)],
            self.get_api_key(),
        )
    }

    fn get_query_types(&self) -> Result<QueryTypes, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("getQueryTypes", "")],
            self.get_api_key(),
        )
    }

    fn get_all_queries(&self, count: u32) -> Result<Vec<Query>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<Query>> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("getAllQueries", count.to_string())],
            self.get_api_key(),
        )?;
        Ok(raw_data.remove("data").unwrap())
    }

    fn enable(&self) -> Result<Status, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php?",
            [("enable", "")],
            self.get_api_key(),
        )
    }

    fn disable(&self, seconds: u64) -> Result<Status, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("disable", seconds.to_string())],
            self.get_api_key(),
        )
    }

    fn get_cache_info(&self) -> Result<CacheInfo, errors::APIError> {
        let mut raw_data: HashMap<String, CacheInfo> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("getCacheInfo", "")],
            self.get_api_key(),
        )?;
        Ok(raw_data.remove("cacheinfo").expect("Missing cache info"))
    }

    fn get_client_names(&self) -> Result<Vec<ClientName>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<ClientName>> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("getClientNames", "")],
            self.get_api_key(),
        )?;
        Ok(raw_data
            .remove("clients")
            .expect("Missing clients attribute"))
    }

    fn get_over_time_data_clients(&self) -> Result<HashMap<String, Vec<u64>>, errors::APIError> {
        let mut raw_data: HashMap<String, FakeHashMap<String, Vec<u64>>> =
            authenticated_json_request(
                self.get_host(),
                "/admin/api.php",
                [("overTimeDataClients", "")],
                self.get_api_key(),
            )?;

        Ok(raw_data
            .remove("over_time")
            .expect("Missing over_time attribute")
            .into())
    }

    fn get_network(&self) -> Result<Network, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api_db.php",
            [("network", "")],
            self.get_api_key(),
        )
    }

    fn get_queries_count(&self) -> Result<u64, errors::APIError> {
        let raw_data: HashMap<String, u64> = authenticated_json_request(
            self.get_host(),
            "/admin/api_db.php",
            [("getQueriesCount", "")],
            self.get_api_key(),
        )?;
        Ok(*raw_data.get("count").expect("Missing count attribute"))
    }

    fn list_add(
        &self,
        domain: &str,
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("add", domain), ("list", list)],
            self.get_api_key(),
        )
    }

    fn list_remove(
        &self,
        domain: &str,
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("sub", domain), ("list", list)],
            self.get_api_key(),
        )
    }

    fn list_get_domains(
        &self,
        list: &str,
    ) -> Result<Vec<CustomListDomainDetails>, errors::APIError> {
        // if not "add" or "sub", api.php defaults to the "get_domains" action
        let mut raw_data: HashMap<String, Vec<CustomListDomainDetails>> =
            authenticated_json_request(
                self.get_host(),
                "/admin/api.php",
                [("get", ""), ("list", list)],
                self.get_api_key(),
            )?;
        Ok(raw_data.remove("data").unwrap())
    }

    fn get_custom_dns_records(&self) -> Result<Vec<CustomDNSRecord>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<Vec<String>>> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("customdns", ""), ("action", "get")],
            self.get_api_key(),
        )?;

        Ok(raw_data
            .remove("data")
            .unwrap()
            .into_iter()
            .map(|list_record| CustomDNSRecord {
                domain: list_record[0].clone(),
                ip_address: list_record[1].parse().unwrap(),
            })
            .collect())
    }

    fn add_custom_dns_record(
        &self,
        ip: &IpAddr,
        domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [
                ("customdns", ""),
                ("action", "add"),
                ("ip", &ip.to_string()),
                ("domain", domain),
            ],
            self.get_api_key(),
        )
    }

    fn delete_custom_dns_record(
        &self,
        ip: &IpAddr,
        domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [
                ("customdns", ""),
                ("action", "delete"),
                ("ip", &ip.to_string()),
                ("domain", domain),
            ],
            self.get_api_key(),
        )
    }

    fn get_custom_cname_records(&self) -> Result<Vec<CustomCNAMERecord>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<Vec<String>>> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("customcname", ""), ("action", "get")],
            self.get_api_key(),
        )?;

        Ok(raw_data
            .remove("data")
            .unwrap()
            .into_iter()
            .map(|list_record| CustomCNAMERecord {
                domain: list_record[0].clone(),
                target_domain: list_record[1].clone(),
            })
            .collect())
    }

    fn add_custom_cname_record(
        &self,
        domain: &str,
        target_domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [
                ("customcname", ""),
                ("action", "add"),
                ("domain", domain),
                ("target", target_domain),
            ],
            self.get_api_key(),
        )
    }

    fn delete_custom_cname_record(
        &self,
        domain: &str,
        target_domain: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [
                ("customcname", ""),
                ("action", "delete"),
                ("domain", domain),
                ("target", target_domain),
            ],
            self.get_api_key(),
        )
    }

    fn get_max_logage(&self) -> Result<f32, errors::APIError> {
        let mut raw_data: HashMap<String, f32> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php",
            [("getMaxlogage", "")],
            self.get_api_key(),
        )?;
        Ok(raw_data.remove("maxlogage").unwrap())
    }
}
