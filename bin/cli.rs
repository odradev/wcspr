use odra::host::{Deployer, HostEnv, InstallConfig, NoArgs, UpgradeConfig};
use odra::prelude::Addressable;
use odra::schema::casper_contract_schema::NamedCLType;
use odra_cli::{ContractProvider, DeployerExt, cspr};
use odra_cli::{
    deploy::DeployScript,
    scenario::{Args, Error, Scenario, ScenarioMetadata},
    CommandArg, DeployedContractsContainer, OdraCli, 
};
use wcspr::wcspr_v1::WCSPRV1;
use wcspr::wcspr_v2::{WCSPRV2, WCSPRV2HostRef, WCSPRV2UpgradeArgs};

// The named key under which the contract will be stored
// under the account's named keys. It was deployed with this named key
// on testnet and mainnet, so we have to use exactly this one.
const NAMED_KEY: &str = "WrappedNativeToken";

/// Deploys contracts and adds it to the container.
pub struct WCSPRV1DeployScript;

impl DeployScript for WCSPRV1DeployScript {
    fn deploy(
        &self,
        env: &HostEnv,
        container: &mut DeployedContractsContainer
    ) -> Result<(), odra_cli::deploy::Error> {
        let cfg = InstallConfig {
            package_named_key: String::from(NAMED_KEY),
            is_upgradable: true,
            allow_key_override: true,
        };
        WCSPRV1::load_or_deploy_with_cfg(
            env, None, NoArgs, cfg, container, cspr!(400))?;

        Ok(())
    }
}

/// A custom scenario that demonstrates how to use the CLI tool with a custom argument.
pub struct UpgradeV1ToV2Scenario;

impl Scenario for UpgradeV1ToV2Scenario {
    fn args(&self) -> Vec<CommandArg> {
        vec![CommandArg::new(
            "chain_name",
            "The name of the blockchain network",
            NamedCLType::String,
        )]
    }

    fn run(
        &self,
        env: &HostEnv,
        container: &DeployedContractsContainer,
        args: Args
    ) -> Result<(), Error> {
        odra_cli::log("Upgrading WCSPRV1 to WCSPRV2...");
        
        // Read a contract reference from the container
        let contract = container.contract_ref::<WCSPRV1>(env)?;
        odra_cli::log(&format!("Current contract: {:?}", contract.address()));

        // Read the argument value
        let chain_name = args.get_single::<String>("chain_name")?;
        odra_cli::log(&format!("Chain name: {}", chain_name));

        // Upgrade.
        env.set_gas(cspr!(500));
        let contract = WCSPRV2::try_upgrade_with_cfg(env, contract.address(), WCSPRV2UpgradeArgs{
            chain_name,
        }, UpgradeConfig {
            package_named_key: String::from(NAMED_KEY),
            allow_key_override: true,
            force_create_upgrade_group: false,
        })?;
        odra_cli::log("Upgrade successful!");

        container.add_contract::<WCSPRV2HostRef>(&contract)?;

        Ok(())
    }
}

impl ScenarioMetadata for UpgradeV1ToV2Scenario {
    const NAME: &'static str = "upgrade_v1_to_v2";
    const DESCRIPTION: &'static str = "Upgrade WCSPRV1 to WCSPRV2.";
}

/// Main function to run the CLI tool.
pub fn main() {
    OdraCli::new()
        .about("CLI tool for abc smart contract")
        .deploy(WCSPRV1DeployScript)
        .contract::<WCSPRV1>()
        .contract::<WCSPRV2>()
        .scenario(UpgradeV1ToV2Scenario)
        .build()
        .run();
}
