use std::fmt::Debug;

use crate::cache::CacheManager;
use crate::cli::args::ForgeArg;
use crate::cmd::CmdLineRunner;
use crate::config::{Config, Settings};

use crate::forge::{Forge, ForgeType};
use crate::install_context::InstallContext;
use crate::toolset::ToolVersion;

#[derive(Debug)]
pub struct VFoxForge {
    fa: ForgeArg,
    remote_version_cache: CacheManager<Vec<String>>,
}

impl Forge for VFoxForge {
    fn get_type(&self) -> ForgeType {
        ForgeType::VFox
    }

    fn fa(&self) -> &ForgeArg {
        &self.fa
    }

    fn get_dependencies(&self, _tv: &ToolVersion) -> eyre::Result<Vec<String>> {
        Ok(vec!["vfox".into()])
    }

    fn list_remote_versions(&self) -> eyre::Result<Vec<String>> {
        self.remote_version_cache
            .get_or_try_init(|| {
                let plugins = vfox::Plugin::list();
                dbg!(plugins);
                unimplemented!();
            })
            .cloned()
    }

    fn install_version_impl(&self, ctx: &InstallContext) -> eyre::Result<()> {
        unimplemented!();
        let config = Config::try_get()?;
        let settings = Settings::get();
        settings.ensure_experimental("go backend")?;

        // if the (semantic) version has no v prefix, add it
        // we allow max. 6 digits for the major version to prevent clashes with Git commit hashes
        let version = if regex!(r"^\d{1,6}(\.\d+)*([+-.].+)?$").is_match(&ctx.tv.version) {
            format!("v{}", ctx.tv.version)
        } else {
            ctx.tv.version.clone()
        };

        CmdLineRunner::new("go")
            .arg("install")
            .arg(&format!("{}@{}", self.name(), version))
            .with_pr(ctx.pr.as_ref())
            .envs(config.env()?)
            .env("GOBIN", ctx.tv.install_path().join("bin"))
            .execute()?;

        Ok(())
    }
}

impl VFoxForge {
    pub fn new(name: String) -> Self {
        let fa = ForgeArg::new(ForgeType::VFox, &name);
        Self {
            remote_version_cache: CacheManager::new(
                fa.cache_path.join("remote_versions.msgpack.z"),
            ),
            fa,
        }
    }
}
