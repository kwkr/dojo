use anyhow::Result;
use clap::Args;
use dojo_world::metadata::dojo_metadata_from_workspace;
use scarb::core::Config;
use starknet::core::types::FieldElement;

use super::options::account::AccountOptions;
use super::options::starknet::StarknetOptions;
use super::options::transaction::TransactionOptions;
use super::options::world::WorldOptions;
use crate::ops::execute;

#[derive(Debug, Args)]
#[command(about = "Execute a system with the given calldata.")]
pub struct ExecuteArgs {
    #[arg(help = "The address of the contract to be executed. Or fully qualified contract name \
                  (ex: dojo_example::actions::actions")]
    pub contract: String,

    #[arg(help = "The name of the entrypoint to be executed.")]
    pub entrypoint: String,

    #[arg(short, long)]
    #[arg(value_delimiter = ',')]
    #[arg(help = "The calldata to be passed to the system. Comma seperated values e.g., \
                  0x12345,0x69420.")]
    pub calldata: Vec<FieldElement>,

    #[command(flatten)]
    pub starknet: StarknetOptions,

    #[command(flatten)]
    pub account: AccountOptions,

    #[command(flatten)]
    pub world: WorldOptions,

    #[command(flatten)]
    pub transaction: TransactionOptions,
}

impl ExecuteArgs {
    pub fn run(self, config: &Config) -> Result<()> {
        let env_metadata = if config.manifest_path().exists() {
            let ws = scarb::ops::read_workspace(config.manifest_path(), config)?;

            dojo_metadata_from_workspace(&ws).and_then(|inner| inner.env().cloned())
        } else {
            None
        };

        config.tokio_handle().block_on(execute::execute(self, env_metadata))
    }
}
