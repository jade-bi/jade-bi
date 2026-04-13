//! 基础设施层模块

pub mod storage;

pub use storage::{create_vault_directory, load_vaults, save_vaults, upsert_vault, vault_directory_exists};
