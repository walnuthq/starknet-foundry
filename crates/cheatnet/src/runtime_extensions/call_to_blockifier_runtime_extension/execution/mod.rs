pub mod cairo1_execution;
pub mod calls;
pub mod cheated_syscalls;
pub mod deprecated;
pub mod entry_point;
pub mod execution_info;
pub mod syscall_hooks;

use blockifier::execution::errors::EntryPointExecutionError;
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use starknet::core::types::Felt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalnutEntryPointExecutionError {
    #[error(transparent)]
    EntryPointExecutionError(#[from] EntryPointExecutionError),
    #[error("EntryPointExecutionErrorWithTraceAndMemory")]
    EntryPointExecutionErrorWithTraceAndMemory {
        error: EntryPointExecutionError,
        vm_trace: Option<Vec<RelocatedTraceEntry>>,
        vm_memory: Vec<Option<Felt>>,
    },
}

pub type WalnutEntryPointExecutionResult<T> = Result<T, WalnutEntryPointExecutionError>;
