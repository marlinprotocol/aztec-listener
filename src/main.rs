#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = 9003;
    let port_clone = port.clone().to_string();

    let listener =
    kalypso_listener::job_creator::JobCreator::simple_listener_for_non_confidential_prover(
        "0x3A4980B3Fda8b1a15E2FA09963d681Ff14632C6f".into(),
        "1".into(),
        "http://88.99.141.248:8545".into(),
        "0x33065ee43dde743065fbeb1753c1d50b20c09007b98ccbced40fe0c7ad166dec".into(),
        "0x6441dcD0f88f70E912A873baaeC5d02e564Ebc78".into(),
        "0x103e9C0e8E0A745A41F8A52142F452E7f8fAaCAd".into(),
        1,
        987,
        format!("http://localhost:{}/", port_clone),
        format!("http://localhost:{}/", port_clone),
        false,
        2
    );

    let _ = listener.run().await;

    println!("All tasks completed or shutdown.");

    Ok(())
}
