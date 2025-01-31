#![feature(slice_take)]

extern crate core;
mod core_range;
pub mod errors;
pub mod manager;
pub mod types;

pub use ccp_shared::types::CUID;
pub use core_range::CoreRange;
pub use cpu_utils::LogicalCoreId;
pub use cpu_utils::PhysicalCoreId;
