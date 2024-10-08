use dotenv::dotenv;

macro_rules! env_var {
    ($var:ident, $key:expr) => {
        let $var = std::env::var($key).expect(&format!("{} is not set", $key));
    };
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    env_var!(generator, "GENERATOR_ADDRESS");
    env_var!(gas_key, "GAS_KEY");
    env_var!(market_id, "MARKET_ID");
    env_var!(http_rpc_url, "HTTP_RPC_URL");
    env_var!(proof_market_place, "PROOF_MARKETPLACE_ADDRESS");
    env_var!(generator_registry, "GENERATOR_REGISTRY_ADDRESS");
    env_var!(start_block, "START_BLOCK");
    env_var!(chain_id, "CHAIN_ID");
    env_var!(max_parallel_proofs, "MAX_PARALLEL_PROOFS");
    env_var!(ivs_url, "IVS_URL");
    env_var!(prover_url, "PROVER_URL");

    let start_block = start_block.parse().expect("Can not parse start_block");
    let chain_id = chain_id.parse().expect("Can not parse chain _id");
    let max_parallel_proofs = max_parallel_proofs.parse().unwrap_or_else(|_| 1);

    let listener =
        kalypso_listener::job_creator::JobCreator::simple_listener_for_non_confidential_prover(
            generator,
            market_id.into(),
            http_rpc_url.into(),
            gas_key,
            proof_market_place.into(),
            generator_registry.into(),
            start_block,
            chain_id,
            prover_url,
            ivs_url,
            false,
            max_parallel_proofs,
        );

    let _ = listener.run().await;

    println!("All tasks completed or shutdown.");

    Ok(())
}
