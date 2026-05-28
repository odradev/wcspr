use odra::host::{HostEnv, InstallConfig, NoArgs};
use odra::schema::casper_contract_schema::NamedCLType;
use odra_cli::{DeployerExt, cspr};
use odra_cli::{
    deploy::DeployScript,
    scenario::{Args, Error, Scenario, ScenarioMetadata},
    CommandArg, DeployedContractsContainer, OdraCli, 
};
use wcspr::wcspr_v1::WCSPRV1;

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
pub struct MyScenario;

impl Scenario for MyScenario {
    fn args(&self) -> Vec<CommandArg> {
        vec![CommandArg::new(
            "my_arg",
            "A custom argument for the scenario",
            NamedCLType::String,
        )]
    }

    fn run(
        &self,
        _env: &HostEnv,
        _container: &DeployedContractsContainer,
        args: Args
    ) -> Result<(), Error> {
        // Read a contract reference from the container
        // let mut contract = container.contract_ref::<MyContract>(env)?;

        // Read the argument value
        let _my_arg = args.get_single::<String>("my_arg")?;

        Ok(())
    }
}

impl ScenarioMetadata for MyScenario {
    const NAME: &'static str = "my_scenario";
    const DESCRIPTION: &'static str = 
        "A custom scenario that demonstrates how to use the CLI tool with a custom argument.";
}

/// Main function to run the CLI tool.
pub fn main() {
    OdraCli::new()
        .about("CLI tool for abc smart contract")
        .deploy(WCSPRV1DeployScript)
        .contract::<WCSPRV1>()
        .scenario(MyScenario)
        .build()
        .run();
}
