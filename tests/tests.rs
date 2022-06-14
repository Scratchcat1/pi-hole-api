use pi_hole_api;
use pi_hole_api::PiHoleAPI;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use test_context::{test_context, TestContext};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn pi_hole_api_test_target() -> String {
    env::var("PI_HOLE_API_TEST_TARGET").expect("Missing environmental var PI_HOLE_API_TEST_TARGET")
}

fn test_target_http_address() -> String {
    env::var("PI_HOLE_API_TEST_TARGET_HTTP_ADDRESS")
        .expect("Missing environmental var PI_HOLE_API_TEST_TARGET_HTTP_ADDRESS")
}

fn test_target_dns_address() -> String {
    env::var("PI_HOLE_API_TEST_TARGET_DNS_ADDRESS")
        .expect("Missing environmental var PI_HOLE_API_TEST_TARGET_DNS_ADDRESS")
}

fn pi_hole_api_test_api_key() -> Option<String> {
    Some(
        env::var("PI_HOLE_API_TEST_API_KEY")
            .expect("Missing environmental var PI_HOLE_API_TEST_API_KEY"),
    )
}

struct PiHoleTestContext {
    resolver: Resolver,
    api: PiHoleAPI,
}

#[async_trait::async_trait]
impl TestContext for PiHoleTestContext {
    fn setup() -> PiHoleTestContext {
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
        let resolver = Resolver::new(resolver_config, ResolverOpts::default()).unwrap();
        let api = PiHoleAPI::new(test_target_http_address(), None);

        PiHoleTestContext { resolver, api }
    }
}

#[test_context(PiHoleTestContext)]
#[test]
fn get_summary_raw_test(ctx: &mut PiHoleTestContext) {
    match ctx.api.get_summary_raw() {
        Ok(summary_raw) => assert!(
            summary_raw.status == "enabled" || summary_raw.status == "disabled",
            "Pi-Hole is neither enabled nor disabled"
        ),
        Err(e) => assert!(false, "Failed to get summary raw: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_summary_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_summary() {
        Ok(summary) => assert!(
            summary.status == "enabled" || summary.status == "disabled",
            "Pi-Hole is neither enabled nor disabled"
        ),
        Err(e) => assert!(false, "Failed to get summary: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_over_time_data_10_mins_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_over_time_data_10_mins() {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get over time data 10 minutes: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_top_items_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_top_items(None) {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };

    match api.get_top_items(Some(1)) {
        Ok(top_items) => {
            assert!(top_items.top_queries.len() <= 1);
        }
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };

    match api.get_top_items(Some(100)) {
        Ok(top_items) => {
            assert!(top_items.top_queries.len() <= 100);
        }
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_top_clients_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_top_clients(None) {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };

    match api.get_top_clients(Some(1)) {
        Ok(top_clients) => {
            assert!(top_clients.top_sources.len() <= 1);
        }
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };

    match api.get_top_clients(Some(100)) {
        Ok(top_clients) => {
            assert!(top_clients.top_sources.len() <= 100);
        }
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_top_clients_blocked_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_top_clients_blocked(None) {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };

    match api.get_top_clients_blocked(Some(1)) {
        Ok(top_clients_blocked) => {
            assert!(top_clients_blocked.top_sources_blocked.len() <= 1);
        }
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };

    match api.get_top_clients_blocked(Some(100)) {
        Ok(top_clients_blocked) => {
            assert!(top_clients_blocked.top_sources_blocked.len() <= 100);
        }
        Err(e) => assert!(false, "Failed to get top items: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_forward_destinations_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_forward_destinations() {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get forward destinations: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_query_types_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_query_types() {
        Ok(query_types) => {
            assert!(query_types.querytypes.get("A (IPv4)").expect("Missing key") >= &0.0);
        }
        Err(e) => assert!(false, "Failed to get query types: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_all_queries_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_all_queries(100) {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get all queries: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn enable_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.enable() {
        Ok(status) => {
            assert!(status.status == "enabled");
        }
        Err(e) => assert!(false, "Failed to enable pi-hole: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn disable_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.disable(10) {
        Ok(status) => {
            assert!(status.status == "disabled");
        }
        Err(e) => assert!(false, "Failed to disable pi-hole: {}", e),
    };
    api.enable().expect("Failed to reenable pi-hole after test");
}
#[test_context(PiHoleTestContext)]
#[test]
fn version_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_version() {
        Ok(version) => {
            assert!(version.version >= 3);
        }
        Err(e) => assert!(false, "Failed to get version: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_cache_info_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_cache_info() {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to get cache info: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_client_names_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_client_names() {
        Ok(client_names) => {
            assert!(client_names.len() > 0);
        }
        Err(e) => assert!(false, "Failed to get client names: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_over_time_data_clients_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_over_time_data_clients() {
        Ok(over_time_data_clients) => {
            assert!(over_time_data_clients.len() > 0);
        }
        Err(e) => assert!(false, "Failed to get over time data clients: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_network_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_network() {
        Ok(network) => {
            assert!(network.network.len() > 0);
        }
        Err(e) => assert!(false, "Failed to get network information: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn get_queries_count_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_queries_count() {
        Ok(count) => {
            assert!(count > 0);
        }
        Err(e) => assert!(false, "Failed to get network information: {}", e),
    };
}
#[test_context(PiHoleTestContext)]
#[test]
fn add_test(ctx: &mut PiHoleTestContext) {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.add(vec!["testdomain.foo"], "white") {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to add domain to list: {}", e),
    };
}
