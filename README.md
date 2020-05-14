# pi-hole-api
Rust library for interacting with the Pi Hole PHP API

## Testing
Testing occurs against a working Pi-Hole, to test run `cargo test`.  
If you don't want to risk altering your Pi-Hole set up another one and alter the `PI_HOLE_API_TEST_TARGET` environmental variable to point at it.  
The API key is required for some tests and is read from the `PI_HOLE_API_TEST_API_KEY` environmental variable.

## TODO
[X] Implement the [V3.0 March 2017 version](https://discourse.pi-hole.net/t/pi-hole-api/1863) of the API
[X] Use environmental variables for the tests  
[ ] Handle errors better when no api key is set