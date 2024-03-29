pub mod code;
pub mod compatibility;
pub mod executor;
mod metadata;

use eyre::{eyre, Result};

use foundry_cli::opts::RpcOpts;
pub use metadata::ClonedProject;

pub async fn tweak(rpc: &RpcOpts, cloned_project: &ClonedProject) -> Result<()> {
    // collect the clone metadata
    let metadata = cloned_project.metadata.clone();

    let output = cloned_project.compile_safe()?;
    let (_, _, artifact) = output
        .artifacts_with_files()
        .find(|(_, contract_name, _)| **contract_name == metadata.target_contract)
        .ok_or_else(|| {
            eyre!("the contract {} is not found in the project", metadata.target_contract)
        })?;

    compatibility::check_storage_compatibility(cloned_project)?;
    code::generate_tweaked_code(rpc, &cloned_project, &artifact).await?;
    Ok(())
}