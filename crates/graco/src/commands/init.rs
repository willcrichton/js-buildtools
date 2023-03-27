use std::{path::Path, str::FromStr};

use crate::{
  utils,
  workspace::{
    package::{Package, PackageName, Platform, Target},
    PackageCommand, Workspace, WorkspaceCommand,
  },
};
use anyhow::Result;

#[derive(clap::Parser, Default)]
pub struct InitArgs {}

pub struct InitCommand {
  #[allow(unused)]
  args: InitArgs,
}

impl InitCommand {
  pub fn new(args: InitArgs) -> Self {
    InitCommand { args }
  }

  fn link_packages(
    &self,
    pkgs_to_link: &[&str],
    local_node_modules: &Path,
    ws: &Workspace,
  ) -> Result<()> {
    let global_node_modules = ws.global_config.node_path();
    for to_link in pkgs_to_link {
      let pkg_name = PackageName::from_str(to_link).unwrap();
      let src = global_node_modules.join(to_link);
      let dst = local_node_modules.join(to_link);
      if let Some(scope) = pkg_name.scope {
        utils::create_dir_if_missing(
          local_node_modules.join(local_node_modules.join(format!("@{scope}"))),
        )?;
      }
      utils::symlink_dir_if_missing(&src, &dst)?;
    }
    Ok(())
  }
}

#[async_trait::async_trait]
impl WorkspaceCommand for InitCommand {
  async fn run(&self, ws: &Workspace) -> Result<()> {
    let local_node_modules = ws.root.join("node_modules");
    utils::create_dir_if_missing(&local_node_modules)?;

    let pkgs_to_link = vec!["@trivago/prettier-plugin-sort-imports"];
    self.link_packages(&pkgs_to_link, &local_node_modules, ws)?;

    Ok(())
  }
}

#[async_trait::async_trait]
impl PackageCommand for InitCommand {
  async fn run(&self, pkg: &Package) -> Result<()> {
    let local_node_modules = pkg.root.join("node_modules");
    utils::create_dir_if_missing(&local_node_modules)?;

    let mut pkgs_to_link = match pkg.target {
      Target::Script => vec!["esbuild"],
      Target::Site => vec!["@vitejs/plugin-react"],
      Target::Lib => vec![],
    };
    match pkg.platform {
      Platform::Browser => pkgs_to_link.extend(["jsdom"]),
      Platform::Node => {}
    }
    pkgs_to_link.extend(["vite", "vitest"]);

    self.link_packages(&pkgs_to_link, &local_node_modules, pkg.workspace())?;

    Ok(())
  }
}
