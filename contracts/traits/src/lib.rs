#![cfg_attr(not(feature = "std"), no_std)]

// =========================================================================
// Existing modules
// =========================================================================
pub mod access_control;
pub mod constants;
pub mod errors;
pub mod i18n;
pub mod monitoring;

// =========================================================================
// New domain-specific modules (Issue #101)
// =========================================================================
pub mod bridge;
pub mod compliance;
pub mod dex;
pub mod fee;
pub mod oracle;
pub mod property;

// =========================================================================
// Re-exports for backward compatibility
// =========================================================================

// Original re-exports
pub use errors::*;
pub use i18n::*;
pub use monitoring::*;

// Re-export all new module contents at the crate root so that
// existing `use propchain_traits::*` continues to resolve every type.
pub use bridge::*;
pub use compliance::*;
pub use dex::*;
pub use fee::*;
pub use oracle::*;
pub use property::*;

#[cfg(not(feature = "std"))]
use scale_info::prelude::vec::Vec;
