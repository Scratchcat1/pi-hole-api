use crate::fake_hash_map::FakeHashMap;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
pub mod api_types;
mod custom_deserializers;
pub mod errors;
mod fake_hash_map;
use crate::api_types::*;

trait PiHoleAPIHost {
    fn get_host(&self) -> &str;
}

trait PiHoleAPIKey {
    fn get_api_key(&self) -> &str;
}

/// Pi Hole API Struct
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

fn simple_json_request<T>(host: &str, path_query: &str) -> Result<T, errors::APIError>
where
    T: DeserializeOwned,
{
    let response = reqwest::blocking::get(&format!("{}{}", host, path_query))?;
    Ok(response.json()?)
}

impl<T> UnauthenticatedPiHoleAPI for T
where
    T: PiHoleAPIHost,
{
    fn get_summary_raw(&self) -> Result<SummaryRaw, errors::APIError> {
        simple_json_request(&self.get_host(), "/admin/api.php?summaryRaw")
    }

    fn get_summary(&self) -> Result<Summary, errors::APIError> {
        simple_json_request(&self.get_host(), "/admin/api.php?summary")
    }

    fn get_over_time_data_10_mins(&self) -> Result<OverTimeData, errors::APIError> {
        simple_json_request(&self.get_host(), "/admin/api.php?overTimeData10mins")
    }

    fn get_version(&self) -> Result<u32, errors::APIError> {
        let raw_version: Version = simple_json_request(&self.get_host(), "/admin/api.php?version")?;
        Ok(raw_version.version)
    }

    fn get_versions(&self) -> Result<Versions, errors::APIError> {
        simple_json_request(&self.get_host(), "/admin/api.php?versions")
    }
}

pub trait AuthenticatedPiHoleAPI {
    /// Get the top domains and ads and the number of queries for each. Limit the number of items with `count`.
    fn get_top_items(&self, count: Option<u32>) -> Result<TopItems, errors::APIError>;

    /// Get the top clients and the number of queries for each. Limit the number of items with `count`.
    fn get_top_clients(&self, count: Option<u32>) -> Result<TopClients, errors::APIError>;

    /// Get the top clients blocked and the number of queries for each. Limit the number of items with `count`.
    fn get_top_clients_blocked(
        &self,
        count: Option<u32>,
    ) -> Result<TopClientsBlocked, errors::APIError>;

    /// Get the number of queries forwarded and the target.
    fn get_forward_destinations(&self) -> Result<ForwardDestinations, errors::APIError>;

    /// Get the number of queries per type.
    fn get_query_types(&self) -> Result<QueryTypes, errors::APIError>;

    /// Get all DNS query data. Limit the number of items with `count`.
    fn get_all_queries(&self, count: u32) -> Result<AllQueries, errors::APIError>;

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
        domains: &[&str],
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError>;

    /// Remove domains to a custom white/blacklist.
    /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
    fn list_remove(
        &self,
        domains: &[&str],
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
}

fn authenticated_json_request<T>(
    host: &str,
    path_query: &str,
    api_key: &str,
) -> Result<T, errors::APIError>
where
    T: DeserializeOwned,
{
    let joining_char = if path_query.contains('?') { '&' } else { '?' };
    let auth_path_query = format!("{}{}{}auth={}", host, path_query, joining_char, api_key);
    let response = reqwest::blocking::get(&auth_path_query)?;
    println!("{:?}", reqwest::blocking::get(&auth_path_query)?.text()?);
    Ok(response.json()?)
}

/// Perform a custom white/blacklist action ("add" or "sub")
fn list_action<T>(
    host: &str,
    api_key: &str,
    domains: &[&str],
    list: &str,
    action: &str,
) -> Result<T, errors::APIError>
where
    T: DeserializeOwned,
{
    let url = format!(
        "{}/admin/api.php?{}={}&list={}&auth={}",
        host,
        action,
        domains.join(" "),
        list,
        api_key
    );

    let response_text = reqwest::blocking::get(&url)?.text()?;
    println!("{}", response_text);
    if response_text.starts_with("Invalid list") {
        Err(errors::APIError::InvalidList)
    } else {
        match serde_json::from_str::<T>(&response_text) {
            Ok(response) => Ok(response),
            Err(error) => Err(error.into()),
        }
    }
}

impl<T> AuthenticatedPiHoleAPI for T
where
    T: PiHoleAPIHost + PiHoleAPIKey,
{
    fn get_top_items(&self, count: Option<u32>) -> Result<TopItems, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            &format!("/admin/api.php?topItems={}", count.unwrap_or(10)),
            self.get_api_key(),
        )
    }

    fn get_top_clients(&self, count: Option<u32>) -> Result<TopClients, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            &format!("/admin/api.php?topClients={}", count.unwrap_or(10)),
            self.get_api_key(),
        )
    }

    fn get_top_clients_blocked(
        &self,
        count: Option<u32>,
    ) -> Result<TopClientsBlocked, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            &format!("/admin/api.php?topClientsBlocked={}", count.unwrap_or(10)),
            self.get_api_key(),
        )
    }

    fn get_forward_destinations(&self) -> Result<ForwardDestinations, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php?getForwardDestinations",
            self.get_api_key(),
        )
    }

    fn get_query_types(&self) -> Result<QueryTypes, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            "/admin/api.php?getQueryTypes",
            self.get_api_key(),
        )
    }

    fn get_all_queries(&self, count: u32) -> Result<AllQueries, errors::APIError> {
        let raw_data: HashMap<String, Vec<Vec<String>>> = authenticated_json_request(
            self.get_host(),
            &format!("/admin/api.php?getAllQueries={}", count),
            self.get_api_key(),
        )?;

        // Convert the queries from a list into a more useful Query struct
        let data = AllQueries {
            data: raw_data
                .get("data")
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

    fn enable(&self) -> Result<Status, errors::APIError> {
        authenticated_json_request(self.get_host(), "/admin/api.php?enable", self.get_api_key())
    }

    fn disable(&self, seconds: u64) -> Result<Status, errors::APIError> {
        authenticated_json_request(
            self.get_host(),
            &format!("/admin/api.php?disable={}", seconds),
            self.get_api_key(),
        )
    }

    fn get_cache_info(&self) -> Result<CacheInfo, errors::APIError> {
        let mut raw_data: HashMap<String, CacheInfo> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php?getCacheInfo",
            self.get_api_key(),
        )?;
        Ok(raw_data.remove("cacheinfo").expect("Missing cache info"))
    }

    fn get_client_names(&self) -> Result<Vec<ClientName>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<ClientName>> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php?getClientNames",
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
                "/admin/api.php?overTimeDataClients",
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
            "/admin/api_db.php?network",
            self.get_api_key(),
        )
    }

    fn get_queries_count(&self) -> Result<u64, errors::APIError> {
        let raw_data: HashMap<String, u64> = authenticated_json_request(
            self.get_host(),
            "/admin/api_db.php?getQueriesCount",
            self.get_api_key(),
        )?;
        Ok(*raw_data.get("count").expect("Missing count attribute"))
    }

    fn list_add(
        &self,
        domains: &[&str],
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        list_action(self.get_host(), self.get_api_key(), domains, list, "add")
    }

    fn list_remove(
        &self,
        domains: &[&str],
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        list_action(self.get_host(), self.get_api_key(), domains, list, "sub")
    }

    fn list_get_domains(
        &self,
        list: &str,
    ) -> Result<Vec<CustomListDomainDetails>, errors::APIError> {
        // if not "add" or "sub", api.php defaults to the "get_domains" action
        let mut raw_data: HashMap<String, Vec<CustomListDomainDetails>> = list_action(
            self.get_host(),
            self.get_api_key(),
            &[],
            list,
            "get_domains",
        )?;
        Ok(raw_data.remove("data").unwrap())
    }

    fn get_custom_dns_records(&self) -> Result<Vec<CustomDNSRecord>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<Vec<String>>> = authenticated_json_request(
            self.get_host(),
            "/admin/api.php?customdns&action=get",
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
}
