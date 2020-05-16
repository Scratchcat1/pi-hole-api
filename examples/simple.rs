use pi_hole_api::PiHoleAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PiHoleAPI::new("http://192.168.0.19".to_string(), None);

    let status = api.get_summary().await?;
    println!("{:?}", status);
    Ok(())
}
