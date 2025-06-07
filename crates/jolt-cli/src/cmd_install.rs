use anyhow::Result;
use jolt_manager::manager::Manager;

pub(crate) trait CmdInstall {
    fn install(&self, distribution: String, version: String) -> Result<()>;
}

impl CmdInstall for Manager {
    fn install(&self, _distribution: String, _version: String) -> Result<()> {
        todo!()
    }
}
