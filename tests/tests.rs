use chrono::prelude::*;
use pi_hole_api;
use pi_hole_api::errors::APIError;
use pi_hole_api::{
    AuthenticatedPiHoleAPI, PiHoleAPIConfig, PiHoleAPIConfigWithKey, UnauthenticatedPiHoleAPI,
};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
// use std::{thread, time};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use test_context::{test_context, TestContext};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

// const DNS_QUERY_DELAY: time::Duration = time::Duration::from_millis(10);

fn test_target_http_address() -> String {
    env::var("PI_HOLE_API_TEST_TARGET_HTTP_ADDRESS")
        .expect("Missing environmental var PI_HOLE_API_TEST_TARGET_HTTP_ADDRESS")
}

fn test_target_dns_address() -> String {
    env::var("PI_HOLE_API_TEST_TARGET_DNS_ADDRESS")
        .expect("Missing environmental var PI_HOLE_API_TEST_TARGET_DNS_ADDRESS")
}

fn pi_hole_api_test_api_key() -> String {
    env::var("PI_HOLE_API_TEST_API_KEY")
        .expect("Missing environmental var PI_HOLE_API_TEST_API_KEY")
}

struct PiHoleTestContext {
    resolver: Resolver,
    authenticated_api: PiHoleAPIConfigWithKey,
    unauthenticated_api: PiHoleAPIConfig,
}

impl PiHoleTestContext {
    pub fn new() -> Self {
        Self {
            resolver: Self::create_resolver(),
            unauthenticated_api: PiHoleAPIConfig::new(test_target_http_address()),
            authenticated_api: PiHoleAPIConfigWithKey::new(
                test_target_http_address(),
                pi_hole_api_test_api_key(),
            ),
        }
    }

    fn create_resolver() -> Resolver {
        let name_server = NameServerConfig {
            socket_addr: SocketAddr::from_str(&test_target_dns_address())
                .expect("Failed to parse test target IP address"),
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_nx_responses: false,
            bind_addr: None,
        };
        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(name_server);
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.cache_size = 0;
        Resolver::new(resolver_config, resolver_opts).unwrap()
    }

    fn lookup_ip(&self, domain: &str) {
        self.resolver.lookup_ip(domain).unwrap();
        // thread::sleep(DNS_QUERY_DELAY);
    }
}

impl TestContext for PiHoleTestContext {
    fn setup() -> PiHoleTestContext {
        PiHoleTestContext::new()
    }
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_summary_raw_test(ctx: &mut PiHoleTestContext) {
    let summary_raw = ctx.unauthenticated_api.get_summary_raw().unwrap();
    assert!(
        summary_raw.status == "enabled" || summary_raw.status == "disabled",
        "Pi-Hole is neither enabled nor disabled"
    );
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_summary_test(ctx: &mut PiHoleTestContext) {
    let summary = ctx.unauthenticated_api.get_summary().unwrap();
    assert!(
        summary.status == "enabled" || summary.status == "disabled",
        "Pi-Hole is neither enabled nor disabled"
    );
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_over_time_data_10_mins_test(ctx: &mut PiHoleTestContext) {
    // Takes a while to update so performing a request will not immediately increase the counter
    ctx.unauthenticated_api
        .get_over_time_data_10_mins()
        .unwrap();
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_top_items_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("google.com");

    let top_items = ctx.authenticated_api.get_top_items(None).unwrap();
    assert!(top_items.top_queries.len() >= 1);

    let top_items = ctx.authenticated_api.get_top_items(Some(1)).unwrap();
    assert_eq!(top_items.top_queries.len(), 1);
    let top_query_count = top_items.top_queries.values().next().unwrap();

    // Test the top query increases
    let top_query = top_items.top_queries.keys().next().unwrap();
    ctx.lookup_ip(top_query);
    let top_items = ctx.authenticated_api.get_top_items(Some(1)).unwrap();
    assert_eq!(
        *top_items.top_queries.values().next().unwrap(),
        *top_query_count + 1
    );

    let top_items = ctx.authenticated_api.get_top_items(Some(100)).unwrap();
    assert!(top_items.top_queries.len() <= 100);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_top_clients_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("google.com");
    let top_clients = ctx.authenticated_api.get_top_clients(None).unwrap();
    assert!(top_clients.top_sources.len() >= 1);

    let top_clients = ctx.authenticated_api.get_top_clients(Some(1)).unwrap();
    assert!(top_clients.top_sources.len() <= 1);

    let top_clients = ctx.authenticated_api.get_top_clients(Some(100)).unwrap();
    assert!(top_clients.top_sources.len() <= 100);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_top_clients_blocked_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("analytics.query.yahoo.com");
    let top_clients_blocked = ctx.authenticated_api.get_top_clients_blocked(None).unwrap();
    assert!(top_clients_blocked.top_sources_blocked.len() >= 1);

    let top_clients_blocked = ctx
        .authenticated_api
        .get_top_clients_blocked(Some(1))
        .unwrap();

    assert!(top_clients_blocked.top_sources_blocked.len() <= 1);
    let top_clients_blocked = ctx
        .authenticated_api
        .get_top_clients_blocked(Some(100))
        .unwrap();

    assert!(top_clients_blocked.top_sources_blocked.len() <= 100);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_forward_destinations_test(ctx: &mut PiHoleTestContext) {
    let forward_destination = ctx
        .authenticated_api
        .get_forward_destinations(false)
        .unwrap();
    assert!(forward_destination.forward_destinations.len() >= 1);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_query_types_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("google.com");
    let query_types = ctx.authenticated_api.get_query_types().unwrap();

    assert!(query_types.querytypes.get("A (IPv4)").expect("Missing key") >= &0.0);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_all_queries_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("google.com");
    let queries = ctx.authenticated_api.get_all_queries(100).unwrap();
    assert!(queries.len() >= 1);
    assert!(queries.iter().any(|query| query.domain == "google.com"));
}

#[test_context(PiHoleTestContext)]
#[test]
fn enable_test(ctx: &mut PiHoleTestContext) {
    let status = ctx.authenticated_api.enable().unwrap();
    assert!(status.status == "enabled");
}

#[test_context(PiHoleTestContext)]
#[test]
fn disable_test(ctx: &mut PiHoleTestContext) {
    let status = ctx.authenticated_api.disable(10).unwrap();
    assert!(status.status == "disabled");

    ctx.authenticated_api
        .enable()
        .expect("Failed to reenable pi-hole after test");
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_version_test(ctx: &mut PiHoleTestContext) {
    let version = ctx.unauthenticated_api.get_version().unwrap();
    assert!(version >= 3);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_versions_test(ctx: &mut PiHoleTestContext) {
    let versions = ctx.unauthenticated_api.get_versions().unwrap();
    assert_ne!(
        versions.core_current == versions.core_latest,
        versions.core_update
    );
    assert_ne!(
        versions.web_current == versions.web_latest,
        versions.web_update
    );
    assert_ne!(
        versions.ftl_current == versions.ftl_latest,
        versions.ftl_update
    );
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_cache_info_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("google.com");
    let cache_info = ctx.authenticated_api.get_cache_info().unwrap();
    assert!(cache_info.cache_inserted > 0);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_client_names_test(ctx: &mut PiHoleTestContext) {
    ctx.lookup_ip("google.com");
    let client_names = ctx.authenticated_api.get_client_names().unwrap();
    assert!(client_names.len() > 0);
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_over_time_data_clients_test(ctx: &mut PiHoleTestContext) {
    ctx.authenticated_api.get_over_time_data_clients().unwrap();
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_network_test(ctx: &mut PiHoleTestContext) {
    // This takes a while to update the DB so testing for change is difficult
    ctx.authenticated_api.get_network().unwrap();
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_queries_count_test(ctx: &mut PiHoleTestContext) {
    // This takes a while to update the DB so testing for change is difficult
    ctx.authenticated_api.get_queries_count().unwrap();
}

#[test_context(PiHoleTestContext)]
#[test]
fn add_test(ctx: &mut PiHoleTestContext) {
    let response = ctx
        .authenticated_api
        .list_add(&["testdomain.foo"], "white")
        .unwrap();
    assert!(response.success);

    let response = ctx
        .authenticated_api
        .list_add(&["testdomain.foo"], "NOT_A_LIST");
    assert!(matches!(response.err().unwrap(), APIError::InvalidList));
}

#[test_context(PiHoleTestContext)]
#[test]
fn remove_test(ctx: &mut PiHoleTestContext) {
    let response = ctx
        .authenticated_api
        .list_remove(&["x.testdomain.foo"], "white")
        .unwrap();
    assert!(response.success);

    let response = ctx
        .authenticated_api
        .list_remove(&["x.testdomain.foo"], "NOT_A_LIST");
    assert!(matches!(response.err().unwrap(), APIError::InvalidList));
}

#[test_context(PiHoleTestContext)]
#[test]
fn list_get_domains_test(ctx: &mut PiHoleTestContext) {
    ctx.authenticated_api
        .list_add(&["testdomain.foo"], "white")
        .unwrap();

    let domains = ctx.authenticated_api.list_get_domains("white").unwrap();
    assert!(domains.len() > 0);
    assert!(domains
        .iter()
        .any(|domain_details| domain_details.domain == "testdomain.foo"));

    // Date before/after which all domains should have been added before
    // This stops an incorrect conversion e.g. parsing milliseconds as seconds
    // and ending up with a date in the 1970s.
    let lower_cutoff_date = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0);
    let upper_cutoff_date = NaiveDate::from_ymd(2030, 1, 1).and_hms(0, 0, 0);
    assert!(domains.iter().all(
        |domain_details| domain_details.date_added > lower_cutoff_date
            && domain_details.date_added < upper_cutoff_date
    ));
    assert!(domains.iter().all(
        |domain_details| domain_details.date_modified > lower_cutoff_date
            && domain_details.date_modified < upper_cutoff_date
    ));

    ctx.authenticated_api
        .list_remove(&["testdomain.foo"], "white")
        .unwrap();

    let domains = ctx.authenticated_api.list_get_domains("white").unwrap();
    assert_eq!(
        domains
            .iter()
            .filter(|domain_details| domain_details.domain == "testdomain.foo")
            .count(),
        0
    );

    let response = ctx.authenticated_api.list_get_domains("NOT_A_LIST");
    assert!(matches!(response.err().unwrap(), APIError::InvalidList));
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_custom_dns_records_test(ctx: &mut PiHoleTestContext) {
    ctx.authenticated_api.get_custom_dns_records().unwrap();
}

#[test_context(PiHoleTestContext)]
#[test]
fn add_and_delete_custom_dns_records_test(ctx: &mut PiHoleTestContext) {
    let ipv4_test_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ipv4_test_domain = "4.example.com";
    let ipv6_test_address = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let ipv6_test_domain = "6.example.com";

    ctx.authenticated_api
        .delete_custom_dns_record(ipv4_test_address, ipv4_test_domain)
        .unwrap();
    ctx.authenticated_api
        .delete_custom_dns_record(ipv6_test_address, ipv6_test_domain)
        .unwrap();

    let custom_dns_records = ctx.authenticated_api.get_custom_dns_records().unwrap();
    assert!(!custom_dns_records
        .iter()
        .any(
            |custom_dns_record| (custom_dns_record.ip_address == ipv4_test_address
                && custom_dns_record.domain == ipv4_test_domain)
                || (custom_dns_record.ip_address == ipv6_test_address
                    && custom_dns_record.domain == ipv6_test_domain)
        ));

    ctx.authenticated_api
        .add_custom_dns_record(ipv4_test_address, ipv4_test_domain)
        .unwrap();
    ctx.authenticated_api
        .add_custom_dns_record(ipv6_test_address, ipv6_test_domain)
        .unwrap();

    let custom_dns_records = ctx.authenticated_api.get_custom_dns_records().unwrap();
    assert_eq!(
        custom_dns_records
            .iter()
            .filter(
                |custom_dns_record| custom_dns_record.ip_address == ipv4_test_address
                    && custom_dns_record.domain == ipv4_test_domain
            )
            .count(),
        1
    );

    assert_eq!(
        custom_dns_records
            .iter()
            .filter(
                |custom_dns_record| custom_dns_record.ip_address == ipv6_test_address
                    && custom_dns_record.domain == ipv6_test_domain
            )
            .count(),
        1
    );
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_custom_cname_records_test(ctx: &mut PiHoleTestContext) {
    ctx.authenticated_api.get_custom_cname_records().unwrap();
}

#[test_context(PiHoleTestContext)]
#[test]
fn add_and_delete_custom_cname_records_test(ctx: &mut PiHoleTestContext) {
    let domain = "abc.example.com";
    let target_domain = "abc.example.net";

    ctx.authenticated_api
        .delete_custom_cname_record(domain, target_domain)
        .unwrap();

    let custom_cname_records = ctx.authenticated_api.get_custom_cname_records().unwrap();
    assert!(!custom_cname_records
        .iter()
        .any(|custom_cname_record| custom_cname_record.domain == domain
            && custom_cname_record.target_domain == target_domain));

    ctx.authenticated_api
        .add_custom_cname_record(domain, target_domain)
        .unwrap();

    let custom_cname_records = ctx.authenticated_api.get_custom_cname_records().unwrap();
    assert_eq!(
        custom_cname_records
            .iter()
            .filter(|custom_cname_record| custom_cname_record.domain == domain
                && custom_cname_record.target_domain == target_domain)
            .count(),
        1
    );
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_max_logage_test(ctx: &mut PiHoleTestContext) {
    let max_logage = ctx.authenticated_api.get_max_logage().unwrap();
    assert!(max_logage >= 0.0);
}
