// Data types for the escrow contract (Issue #101 - extracted from lib.rs)

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub enum EscrowStatus {
    Created,
    Funded,
    Active,
    Released,
    Refunded,
    Disputed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub enum ApprovalType {
    Release,
    Refund,
    EmergencyOverride,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub struct EscrowData {
    pub id: u64,
    pub property_id: u64,
    pub buyer: AccountId,
    pub seller: AccountId,
    pub amount: u128,
    pub deposited_amount: u128,
    pub status: EscrowStatus,
    pub created_at: u64,
    pub release_time_lock: Option<u64>,
    pub participants: Vec<AccountId>,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub struct MultiSigConfig {
    pub required_signatures: u8,
    pub signers: Vec<AccountId>,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub struct DocumentHash {
    pub hash: Hash,
    pub document_type: String,
    pub uploaded_by: AccountId,
    pub uploaded_at: u64,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub struct Condition {
    pub id: u64,
    pub description: String,
    pub met: bool,
    pub verified_by: Option<AccountId>,
    pub verified_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub struct DisputeInfo {
    pub escrow_id: u64,
    pub raised_by: AccountId,
    pub reason: String,
    pub raised_at: u64,
    pub resolved: bool,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink::storage::traits::StorageLayout)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub actor: AccountId,
    pub action: String,
    pub details: String,
}

pub type SignatureKey = (u64, ApprovalType, AccountId);
