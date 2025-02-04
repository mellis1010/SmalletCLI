use anchor_client::Cluster;
use anyhow::{format_err, Result};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::{config::Config, solana_cmd::new_solana_cmd, utils::exec_command};

#[derive(Clone, Debug, Default)]
pub struct Workspace {
    pub path: PathBuf,
    pub cfg: Config,
}

impl Workspace {
    pub fn load(path: &Path) -> Result<Workspace> {
        let cfg = Config::discover()?.expect("Cosmic.toml not found; please run `cosmic init`");
        Ok(Workspace {
            path: path.into(),
            cfg: cfg.into_inner(),
        })
    }

    pub fn reload(&self) -> Result<Workspace> {
        Workspace::load(&self.path)
    }

    pub fn deployer_dir(&self) -> PathBuf {
        self.path.join("deployers/")
    }

    pub fn get_deployer_kp_path(&self, cluster: &Cluster) -> PathBuf {
        let deployer_dir = self.deployer_dir();
        deployer_dir.join(format!("{}.json", cluster))
    }

    pub fn get_deployer_kp_path_if_exists(&self, cluster: &Cluster) -> Result<PathBuf> {
        let deployer_dir = self.deployer_dir();
        if !deployer_dir.exists() {
            return Err(format_err!(
                "{} does not exist; you may need to run `cosmic init`",
                deployer_dir.display()
            ));
        }
        let deployer_kp_path = deployer_dir.join(format!("{}.json", cluster));
        if !deployer_kp_path.exists() {
            return Err(format_err!(
                "Deployer not found at {}; you may need to run `cosmic init`",
                deployer_kp_path.display()
            ));
        }
        Ok(deployer_kp_path)
    }

    pub fn add_cluster_args(&self, command: &mut Command, cluster: &Cluster) -> Result<()> {
        let kp_path = self.get_deployer_kp_path_if_exists(cluster)?;
        command
            .args(["--url", self.get_cluster_url(cluster)?, "--keypair"])
            .arg(kp_path);
        Ok(())
    }

    pub fn exec_deployer_command<F>(&self, cluster: &Cluster, mut builder: F) -> Result<Output>
    where
        F: FnMut(&mut Command) -> Result<()>,
    {
        let cmd = &mut new_solana_cmd();
        self.add_cluster_args(cmd, cluster)?;
        builder(cmd)?;
        exec_command(cmd)
    }

    /// Gets the configured URL of the [Cluster] in `Cosmic.toml`.
    pub fn get_cluster_url(&self, cluster: &Cluster) -> Result<&str> {
        let Workspace { cfg, .. } = self;
        Ok(match cluster {
            Cluster::Debug => &cfg.rpc_endpoints.debug,
            Cluster::Testnet => &cfg.rpc_endpoints.testnet,
            Cluster::Mainnet => &cfg.rpc_endpoints.mainnet,
            Cluster::Devnet => &cfg.rpc_endpoints.devnet,
            Cluster::Localnet => &cfg.rpc_endpoints.localnet,
            _ => panic!("cluster type not supported"),
        })
    }

    /// The upgrader.
    pub fn get_upgrader_wallet(&self) -> Result<String> {
        self.cfg
            .upgrade_authority_keypair
            .clone()
            .ok_or_else(|| format_err!("upgrade_authority_keypair not found in Cosmic.toml"))
    }

    /// New [CommandContext] using the specified cluster.
    pub fn new_cluster_context<'a, 'b>(
        &'a self,
        cluster: &'b Cluster,
    ) -> Result<CommandContext<'a, 'b>> {
        Ok(CommandContext {
            workspace: self,
            cluster,
        })
    }
}

pub struct CommandContext<'a, 'b> {
    pub workspace: &'a Workspace,
    pub cluster: &'b Cluster,
}

impl<'a, 'b> CommandContext<'a, 'b> {
    fn add_cluster_args(&self, command: &mut Command, wallet: &str) -> Result<()> {
        command
            .args([
                "--url",
                self.workspace.get_cluster_url(self.cluster)?,
                "--keypair",
            ])
            .arg(wallet);
        Ok(())
    }

    /// Executes a command.
    pub fn exec_command<F>(&self, mut builder: F, wallet: &str) -> Result<Output>
    where
        F: FnMut(&mut Command) -> Result<()>,
    {
        let cmd = &mut new_solana_cmd();
        self.add_cluster_args(cmd, wallet)?;
        builder(cmd)?;
        exec_command(cmd)
    }

    /// Executes a command.
    pub fn exec_args<S>(&self, args: &[S], wallet: &str) -> Result<Output>
    where
        S: AsRef<OsStr>,
    {
        let cmd = &mut new_solana_cmd();
        self.add_cluster_args(cmd, wallet)?;
        args.iter().for_each(|arg| {
            cmd.arg(arg.as_ref());
        });
        exec_command(cmd)
    }

    pub fn get_deployer_kp_path(&self) -> PathBuf {
        self.workspace.get_deployer_kp_path(self.cluster)
    }

    pub fn parse_wallet_alias(&self, alias: &str) -> Result<String> {
        let result = match alias {
            "deployer" => Ok(self.get_deployer_kp_path().display().to_string()),
            "upgrader" => self.workspace.get_upgrader_wallet(),
            _ => Ok(alias.to_string()),
        };
        result.map_err(|err| format_err!("could not parse alias {}: {}", alias, err))
    }
}
