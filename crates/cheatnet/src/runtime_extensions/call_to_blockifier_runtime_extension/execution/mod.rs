pub mod cairo1_execution;
pub mod calls;
pub mod cheated_syscalls;
pub mod deprecated;
pub mod entry_point;
pub mod execution_info;
pub mod syscall_hooks;

use crate::runtime_extensions::call_to_blockifier_runtime_extension::execution::entry_point::EntryPointExecutionErrorWithTraceAndMemory;
use blockifier::execution::errors::EntryPointExecutionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalnutEntryPointExecutionError {
    #[error(transparent)]
    EntryPointExecutionError(#[from] EntryPointExecutionError),
    #[error(transparent)]
    EntryPointExecutionErrorWithTraceAndMemory(#[from] EntryPointExecutionErrorWithTraceAndMemory),
}

pub type WalnutEntryPointExecutionResult<T> = Result<T, WalnutEntryPointExecutionError>;
