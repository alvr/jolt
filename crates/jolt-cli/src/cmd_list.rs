use anyhow::Result;
use jolt_manager::manager::Manager;

pub(crate) trait CmdList {
    fn list(&self, distribution: Option<String>, version: Option<String>) -> Result<()>;
}

impl CmdList for Manager {
    fn list(&self, _distribution: Option<String>, _version: Option<String>) -> Result<()> {
        todo!()
    }
}
