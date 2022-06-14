use pi_hole_api;
use pi_hole_api::PiHoleAPI;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
// use std::{thread, time};
use test_context::{test_context, TestContext};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

// const DNS_QUERY_DELAY: time::Duration = time::Duration::from_millis(10);

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
    authenticated_api: PiHoleAPI,
    unauthenticated_api: PiHoleAPI,
}

impl PiHoleTestContext {
    pub fn new() -> Self {
        Self {
            resolver: Self::create_resolver(),
            unauthenticated_api: PiHoleAPI::new(test_target_http_address(), None),
            authenticated_api: PiHoleAPI::new(
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
    ctx.lookup_ip("google.com");
    let over_time_data = ctx
        .unauthenticated_api
        .get_over_time_data_10_mins()
        .unwrap();
    assert!(over_time_data.domains_over_time.len() >= 1);
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
    let forward_destination = ctx.authenticated_api.get_forward_destinations().unwrap();
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
    assert!(queries.data.len() >= 1);
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
