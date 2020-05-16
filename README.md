# Pi Hole API
Rust library for interacting with the Pi Hole PHP API.

## Example
### Simple
```rust
use pi_hole_api::PiHoleAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PiHoleAPI::new("http://192.168.0.100".to_string(), None);

    let status = api.get_summary().await?;
    println!("{:?}", status);
    Ok(())
}
```

### Authentication
```rust
use pi_hole_api::PiHoleAPI;

#[tokio::main]
async fn main() {
    // Replace the address and key with those of your Pi Hole
    let api = PiHoleAPI::new(
        "http://192.168.0.100".to_string(),
        Some("YOUR_API_KEY".to_string()),
    );

    match api.get_queries_count().await {
        Ok(status) => println!("Total number of queries: {:?}", status),
        Err(e) => panic!("Request failed, check your address and api key: {:?}", e),
    };
}

```

## Limitations
Only Pi-Hole v5.0+ is supported due to changes in the return types of the API.  
Currently removing domains from blacklists/whitelists via the API is [broken](https://github.com/pi-hole/AdminLTE/issues/1297) and therefore isn't implemented.

## Testing
Testing occurs against a working Pi-Hole, to test run `cargo test`.  
If you don't want to risk altering your Pi-Hole set up another one and alter the `PI_HOLE_API_TEST_TARGET` environmental variable to point at it.  
The API key is required for some tests and is read from the `PI_HOLE_API_TEST_API_KEY` environmental variable.