use pi_hole_api::{PiHoleAPIConfig, UnauthenticatedPiHoleAPI};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PiHoleAPIConfig::new("http://192.168.0.19".to_string());

    let status = api.get_summary();
    println!("{:?}", status);
    Ok(())
}
