mod solidity_bdd;
use std::{env};


#[tokio::main]
// Test runner
async fn main()
{
    // Read fork endpoint from environment variable ETH_NODE_URL
    // FIXME: Default to no fork
    let fork_endpoint = env::var("ETH_NODE_URL").expect("Environment variable ETH_NODE_URL should be defined and be a valid API URL");

    let config = solidity_bdd::init(fork_endpoint, 31337_u64);  // FIXME: Constant
    solidity_bdd::run(config).await
}