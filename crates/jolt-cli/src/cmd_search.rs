use anyhow::Result;
use jolt_manager::manager::Manager;

pub(crate) trait CmdSearch {
    fn search(&self, distribution: Option<String>, version: Option<String>) -> Result<()>;
}

impl CmdSearch for Manager {
    fn search(&self, _distribution: Option<String>, _version: Option<String>) -> Result<()> {
        todo!()
    }
}
