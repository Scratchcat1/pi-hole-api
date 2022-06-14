use pi_hole_api::PiHoleAPI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PiHoleAPI::new("http://192.168.0.19".to_string(), None);

    let status = api.get_summary();
    println!("{:?}", status);
    Ok(())
}
