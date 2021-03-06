use anyhow::{Result, Error};
use structopt::StructOpt;
use crate::cmd::Cmd;
use crate::context::Context;
use crate::manifest::{Layout, Git, DoveToml, Package, Dependence};
use serde::Serialize;

/// Metadata project command.
#[derive(StructOpt, Debug)]
pub struct Metadata {
    #[structopt(short = "j", long = "json")]
    json: bool,
}

impl Cmd for Metadata {
    fn apply(self, mut ctx: Context) -> Result<(), Error> {
        if self.json {
            ctx.manifest.package.name = Some(ctx.project_name());
            println!(
                "{}",
                serde_json::to_string_pretty::<DoveJson>(&ctx.manifest.into())?
            );
        } else {
            println!("{}", toml::to_string_pretty(&ctx.manifest)?);
        }

        Ok(())
    }
}

/// Movec manifest.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DoveJson {
    /// Project info.
    pub package: PackageJson,
    /// Project layout.
    #[serde(default)]
    pub layout: Layout,
}

/// Project info.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PackageJson {
    /// Project name.
    pub name: String,
    /// Project AccountAddress.
    #[serde(default = "code_code_address")]
    pub account_address: Option<String>,
    /// Authors list.
    #[serde(default)]
    pub authors: Vec<String>,
    /// dnode base url.
    pub blockchain_api: Option<String>,
    /// Git dependency list.
    pub git_dependencies: Vec<Git>,
    /// Local dependency list.
    pub local_dependencies: Vec<String>,
}

impl From<DoveToml> for DoveJson {
    fn from(toml: DoveToml) -> Self {
        DoveJson {
            package: toml.package.into(),
            layout: toml.layout,
        }
    }
}

impl From<Package> for PackageJson {
    fn from(pac: Package) -> Self {
        let (locals, git) = if let Some(dependencies) = pac.dependencies {
            dependencies.deps.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut locals, mut gits), elt| {
                    match elt {
                        Dependence::Git(git) => gits.push(git),
                        Dependence::Path(path) => locals.push(path.path),
                    }
                    (locals, gits)
                },
            )
        } else {
            (vec![], vec![])
        };

        PackageJson {
            name: pac.name.unwrap_or_default(),
            account_address: pac.account_address,
            authors: pac.authors,
            blockchain_api: pac.blockchain_api,
            git_dependencies: git,
            local_dependencies: locals,
        }
    }
}
