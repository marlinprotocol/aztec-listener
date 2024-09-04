use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = 9003;
    let port_clone = port.clone().to_string();

    let generator_address = env::var("GENERATOR").expect("GENERATOR is not set");
    let gas_key = env::var("GAS_KEY").expect("GAS_KEY is not set");

    let listener =
        kalypso_listener::job_creator::JobCreator::simple_listener_for_non_confidential_prover(
            generator_address,
            "1".into(),
            "http://88.99.141.248:8545".into(),
            gas_key,
            "0x6441dcD0f88f70E912A873baaeC5d02e564Ebc78".into(),
            "0x103e9C0e8E0A745A41F8A52142F452E7f8fAaCAd".into(),
            1,
            987,
            format!("http://localhost:{}/", port_clone),
            format!("http://localhost:{}/", port_clone),
            false,
            2,
        );

    let _ = listener.run().await;

    println!("All tasks completed or shutdown.");

    Ok(())
}
