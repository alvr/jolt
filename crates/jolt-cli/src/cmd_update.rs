use anyhow::Result;
use jolt_manager::manager::Manager;

pub(crate) trait CmdUpdate {
    fn update(&self) -> Result<()>;
}

impl CmdUpdate for Manager {
    fn update(&self) -> Result<()> {
        todo!()
    }
}
