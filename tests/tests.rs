use pi_hole_api;
use std::env;
use tokio;

fn pi_hole_api_test_target() -> String {
    env::var("PI_HOLE_API_TEST_TARGET").expect("Missing environmental var PI_HOLE_API_TEST_TARGET")
}

fn pi_hole_api_test_api_key() -> Option<String> {
    Some(
        env::var("PI_HOLE_API_TEST_API_KEY")
            .expect("Missing environmental var PI_HOLE_API_TEST_API_KEY"),
    )
}

#[tokio::test]
async fn get_summary_raw_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_summary_raw().await {
        Ok(summary_raw) => assert!(
            summary_raw.status == "enabled" || summary_raw.status == "disabled",
            "Pi-Hole is neither enabled nor disabled"
        ),
        Err(e) => assert!(false, format!("Failed to get summary raw: {}", e)),
    };
}

#[tokio::test]
async fn get_summary_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_summary().await {
        Ok(summary) => assert!(
            summary.status == "enabled" || summary.status == "disabled",
            "Pi-Hole is neither enabled nor disabled"
        ),
        Err(e) => assert!(false, format!("Failed to get summary: {}", e)),
    };
}

#[tokio::test]
async fn get_over_time_data_10_mins_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_over_time_data_10_mins().await {
        Ok(_) => {}
        Err(e) => assert!(
            false,
            format!("Failed to get over time data 10 minutes: {}", e)
        ),
    };
}

#[tokio::test]
async fn get_top_items_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_top_items(None).await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };

    match api.get_top_items(Some(1)).await {
        Ok(top_items) => {
            assert!(top_items.top_queries.len() <= 1);
        }
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };

    match api.get_top_items(Some(100)).await {
        Ok(top_items) => {
            assert!(top_items.top_queries.len() <= 100);
        }
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };
}

#[tokio::test]
async fn get_top_clients_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_top_clients(None).await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };

    match api.get_top_clients(Some(1)).await {
        Ok(top_clients) => {
            assert!(top_clients.top_sources.len() <= 1);
        }
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };

    match api.get_top_clients(Some(100)).await {
        Ok(top_clients) => {
            assert!(top_clients.top_sources.len() <= 100);
        }
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };
}

#[tokio::test]
async fn get_top_clients_blocked_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_top_clients_blocked(None).await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };

    match api.get_top_clients_blocked(Some(1)).await {
        Ok(top_clients_blocked) => {
            assert!(top_clients_blocked.top_sources_blocked.len() <= 1);
        }
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };

    match api.get_top_clients_blocked(Some(100)).await {
        Ok(top_clients_blocked) => {
            assert!(top_clients_blocked.top_sources_blocked.len() <= 100);
        }
        Err(e) => assert!(false, format!("Failed to get top items: {}", e)),
    };
}

#[tokio::test]
async fn get_forward_destinations_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_forward_destinations().await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to get forward destinations: {}", e)),
    };
}

#[tokio::test]
async fn get_query_types_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_query_types().await {
        Ok(query_types) => {
            assert!(query_types.querytypes.get("A (IPv4)").expect("Missing key") >= &0.0);
        }
        Err(e) => assert!(false, format!("Failed to get query types: {}", e)),
    };
}

#[tokio::test]
async fn get_all_queries_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_all_queries(100).await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to get all queries: {}", e)),
    };
}

#[tokio::test]
async fn enable_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.enable().await {
        Ok(status) => {
            assert!(status.status == "enabled");
        }
        Err(e) => assert!(false, format!("Failed to enable pi-hole: {}", e)),
    };
}

#[tokio::test]
async fn disable_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.disable(10).await {
        Ok(status) => {
            assert!(status.status == "disabled");
        }
        Err(e) => assert!(false, format!("Failed to disable pi-hole: {}", e)),
    };
    api.enable()
        .await
        .expect("Failed to reenable pi-hole after test");
}

#[tokio::test]
async fn version_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), None);
    match api.get_version().await {
        Ok(version) => {
            assert!(version.version >= 3);
        }
        Err(e) => assert!(false, format!("Failed to get version: {}", e)),
    };
}

#[tokio::test]
async fn get_cache_info_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_cache_info().await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to get cache info: {}", e)),
    };
}

#[tokio::test]
async fn get_client_names_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_client_names().await {
        Ok(client_names) => {
            assert!(client_names.len() > 0);
        }
        Err(e) => assert!(false, format!("Failed to get client names: {}", e)),
    };
}

#[tokio::test]
async fn get_over_time_data_clients_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_over_time_data_clients().await {
        Ok(over_time_data_clients) => {
            assert!(over_time_data_clients.len() > 0);
        }
        Err(e) => assert!(
            false,
            format!("Failed to get over time data clients: {}", e)
        ),
    };
}

#[tokio::test]
async fn get_network_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_network().await {
        Ok(network) => {
            assert!(network.network.len() > 0);
        }
        Err(e) => assert!(false, format!("Failed to get network information: {}", e)),
    };
}

#[tokio::test]
async fn get_queries_count_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.get_queries_count().await {
        Ok(count) => {
            assert!(count > 0);
        }
        Err(e) => assert!(false, format!("Failed to get network information: {}", e)),
    };
}

#[tokio::test]
async fn add_test() {
    let api = pi_hole_api::PiHoleAPI::new(pi_hole_api_test_target(), pi_hole_api_test_api_key());
    match api.add(vec!["testdomain.foo"], "white").await {
        Ok(_) => {}
        Err(e) => assert!(false, format!("Failed to add domain to list: {}", e)),
    };
}
