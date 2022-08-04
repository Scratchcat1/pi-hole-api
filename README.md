# Pi Hole API
Rust library for interacting with the Pi Hole PHP API.

## Example
### Simple
```rust
use pi_hole_api::{PiHoleAPIConfig, UnauthenticatedPiHoleAPI};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PiHoleAPIConfig::new("http://192.168.0.19".to_string());

    let status = api.get_summary();
    println!("{:?}", status);
    Ok(())
}

```

### Authentication
```rust
use pi_hole_api::{AuthenticatedPiHoleAPI, PiHoleAPIConfigWithKey};

fn main() {
    // Replace the address and key with those of your Pi Hole
    let api = PiHoleAPIConfigWithKey::new(
        "http://192.168.0.100".to_string(),
        "0123456789abcedf0123456789abcedf0123456789abcedf0123456789abcedf".to_string(),
    );

    match api.get_queries_count() {
        Ok(status) => println!("Total number of queries: {:?}", status),
        Err(e) => panic!("Request failed, check your address and api key: {:?}", e),
    };
}

```

## Limitations
~~Only Pi-Hole v5.0+ is supported due to changes in the return types of the API.  
Currently removing domains from blacklists/whitelists via the API is [broken](https://github.com/pi-hole/AdminLTE/issues/1297) and therefore isn't implemented.~~ Resolved: [PR](https://github.com/pi-hole/AdminLTE/pull/1387)

## Testing
The docker-compose file creates a Pi-Hole instance. You will need the API key of the instance to run the test. Store the key in the environment variable `PI_HOLE_API_TEST_API_KEY`.

Environmental variables `PI_HOLE_API_TEST_TARGET_HTTP_ADDRESS` and `PI_HOLE_API_TEST_TARGET_DNS_ADDRESS` should contain the http address (e.g. `http://localhost`) and the DNS IP:Port pair (e.g. `127.0.0.1:53`).

An envrc example with these variables is available in `.envrc-example`.

Once the environmental variables are configured the tests can be run with `cargo test`.