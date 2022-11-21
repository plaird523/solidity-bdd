use std::{env, fs};
use std::collections::HashMap;
use ethers::utils::{Anvil, AnvilInstance};
use ethers_providers::{Provider, Http, Middleware};
use ethers::prelude::*;
use std::time::Duration;
use std::sync::Arc;
use ethers_core::utils::get_contract_address;
use crate::k256::elliptic_curve::weierstrass::add;

pub(crate) struct CucumberRun {

}

pub(crate) struct TestConfig {
    fork_endpoint: String,
    fork_chain_id: u64,
    contracts: Vec<ethers::contracts::Contract<M>>,
    addresses: HashMap<String, ethers::core::types::Address>,
}

pub(crate) trait ContractTestHelper {
    async fn run(&self) -> ();
}

pub(crate) struct SmartContractInfo {
    filepath: String,
    name: String,
}

pub fn init(fork_endpoint: String, fork_chain_id: u64) -> TestConfig { // FIXME: Better options system
    TestConfig(fork_endpoint, fork_chain_id, contracts: Vec::new(), addresses: HashMap::new())
}

// impl ContractTestHelper for TestConfig {
pub async fn run(config: TestConfig) -> () {
        let file = fs::File::create(dbg!(format!("{}/junit.xml", env!("OUT_DIR")))).expect("File should be found");
        //let world =
        WriterWorld::cucumber()
            // Start a fresh anvil before each scenario
            .before(move |_, _, _, world| {
                async move {
                    let fork_endpoint = env::var("ETH_NODE_URL").expect("Environment variable ETH_NODE_URL should be defined and be a valid API URL");
                    let anvil = Anvil::new()
                        .fork(config.fork_endpoint)
                        .chain_id(config.fork_chain_id)
                        .spawn();
                    let endpoint = anvil.endpoint();
                    println!("Anvil running at `{}` with chain_id `{}`", endpoint, anvil.chain_id());

                    let provider = Provider::<Http>::try_from(anvil.endpoint()).expect("Failed to connect to Anvil").interval(Duration::from_millis(10u64));

                    let wallet: LocalWallet = anvil.keys()[0].clone().into();
                    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id())));
                    let connection = AnvilConnection {
                        anvil,
                        client
                    };
                    world.anvil = Option::Some(connection);
                }.boxed()
            })
            .with_writer(writer::JUnit::new(file, 0))    // Uncomment for output to JUnit XML for Github Actions, etc
            .run("tests/features/implemented")
            .await;
    }
// }