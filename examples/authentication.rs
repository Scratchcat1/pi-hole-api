use pi_hole_api::PiHoleAPI;

fn main() {
    // Replace the address and key with those of your Pi Hole
    let api = PiHoleAPI::new(
        "http://192.168.0.100".to_string(),
        Some("0123456789abcedf0123456789abcedf0123456789abcedf0123456789abcedf".to_string()),
    );

    match api.get_queries_count() {
        Ok(status) => println!("Total number of queries: {:?}", status),
        Err(e) => panic!("Request failed, check your address and api key: {:?}", e),
    };
}
