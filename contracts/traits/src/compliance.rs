//! Compliance, regulatory, and structured logging types and traits.
//!
//! This module contains types for compliance operations, the compliance
//! checker trait, and structured logging primitives for event classification.

// =========================================================================
// Compliance Types
// =========================================================================

/// Transaction type for compliance rules engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum ComplianceOperation {
    RegisterProperty,
    TransferProperty,
    UpdateMetadata,
    CreateEscrow,
    ReleaseEscrow,
    ListForSale,
    Purchase,
    BridgeTransfer,
}

/// Trait for compliance registry (used by PropertyRegistry for automated checks)
#[ink::trait_definition]
pub trait ComplianceChecker {
    /// Returns true if the account meets current compliance requirements
    #[ink(message)]
    fn is_compliant(&self, account: ink::primitives::AccountId) -> bool;
}

// =========================================================================
// Structured Logging Types (Issue #107)
// =========================================================================

/// Log severity levels for classifying contract events.
/// Used by off-chain tooling to filter and prioritize event streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum LogLevel {
    /// Informational events: resource creation, normal state transitions
    Info,
    /// Warning events: unusual conditions that may need attention
    Warning,
    /// Error events: operation failures, rejected transactions
    Error,
    /// Critical events: security-related, admin changes, emergency actions
    Critical,
}

/// Event categories for structured log aggregation and filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum EventCategory {
    /// Resource creation: property registered, escrow created, token minted
    Lifecycle,
    /// State mutations: transfers, metadata updates, status changes
    StateChange,
    /// Permission changes: approvals granted or revoked
    Authorization,
    /// Value movements: escrow releases, refunds, fee payments
    Financial,
    /// System operations: pause, resume, upgrades, config changes
    Administrative,
    /// Regulatory and compliance: verification, audit logs, consent
    Audit,
}
