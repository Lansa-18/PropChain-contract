#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod compliance;
mod legal;
mod optimization;
mod payments;
mod tax_engine;

use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::{env::DefaultEnvironment, env::Environment};
use propchain_traits::ComplianceChecker;
use propchain_traits::*;

type AccountId = <DefaultEnvironment as Environment>::AccountId;
type Balance = <DefaultEnvironment as Environment>::Balance;
type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

const BASIS_POINTS_DENOMINATOR: Balance = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Jurisdiction {
    pub code: u32,
    pub country_code: [u8; 2],
    pub region_code: u16,
    pub locality_code: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum ReportingFrequency {
    Monthly,
    Quarterly,
    Annual,
}

impl ReportingFrequency {
    pub(crate) fn period_millis(&self) -> u64 {
        match self {
            Self::Monthly => 30 * 24 * 60 * 60 * 1_000,
            Self::Quarterly => 90 * 24 * 60 * 60 * 1_000,
            Self::Annual => 365 * 24 * 60 * 60 * 1_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct JurisdictionProfile {
    pub surcharge_basis_points: u32,
    pub early_payment_discount_basis_points: u32,
    pub late_payment_grace_period: u64,
    pub optimization_window: u64,
    pub requires_digital_stamp: bool,
    pub authority_hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct TaxRule {
    pub rate_basis_points: u32,
    pub fixed_charge: Balance,
    pub exemption_amount: Balance,
    pub payment_due_period: u64,
    pub reporting_frequency: ReportingFrequency,
    pub penalty_basis_points: u32,
    pub requires_reporting: bool,
    pub requires_legal_documents: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct PropertyAssessment {
    pub owner: AccountId,
    pub assessed_value: Balance,
    pub exemption_override: Balance,
    pub last_assessed_at: Timestamp,
    pub legal_documents_verified: bool,
    pub reporting_submitted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum TaxStatus {
    Assessed,
    PartiallyPaid,
    Paid,
    Overdue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct TaxBreakdown {
    pub taxable_value: Balance,
    pub base_tax: Balance,
    pub fixed_charge: Balance,
    pub surcharge_amount: Balance,
    pub discount_amount: Balance,
    pub penalty_amount: Balance,
    pub total_due: Balance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct TaxRecord {
    pub property_id: u64,
    pub jurisdiction_code: u32,
    pub reporting_period: u64,
    pub assessed_value: Balance,
    pub taxable_value: Balance,
    pub tax_due: Balance,
    pub paid_amount: Balance,
    pub penalty_amount: Balance,
    pub discount_amount: Balance,
    pub due_at: Timestamp,
    pub last_payment_at: Timestamp,
    pub status: TaxStatus,
    pub payment_reference: [u8; 32],
    pub report_hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct PaymentReceipt {
    pub property_id: u64,
    pub jurisdiction_code: u32,
    pub reporting_period: u64,
    pub payment_reference: [u8; 32],
    pub amount_paid: Balance,
    pub outstanding_balance: Balance,
    pub settled_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum LegalDocumentType {
    TitleDeed,
    OccupancyCertificate,
    TaxClearanceCertificate,
    EnvironmentalPermit,
    CorporateResolution,
}

impl LegalDocumentType {
    pub(crate) fn key(&self) -> u8 {
        match self {
            Self::TitleDeed => 0,
            Self::OccupancyCertificate => 1,
            Self::TaxClearanceCertificate => 2,
            Self::EnvironmentalPermit => 3,
            Self::CorporateResolution => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum LegalDocumentStatus {
    Pending,
    Verified,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct LegalDocumentRecord {
    pub property_id: u64,
    pub jurisdiction_code: u32,
    pub document_type: LegalDocumentType,
    pub document_hash: [u8; 32],
    pub issued_at: Timestamp,
    pub expires_at: Timestamp,
    pub verified_at: Timestamp,
    pub status: LegalDocumentStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum ComplianceAlertLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum ComplianceAlertType {
    PaymentDueSoon,
    ReportingMissing,
    LegalDocumentsMissing,
    TaxOverdue,
    RegistryNonCompliant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ComplianceAlert {
    pub property_id: u64,
    pub jurisdiction_code: u32,
    pub reporting_period: u64,
    pub alert_type: ComplianceAlertType,
    pub level: ComplianceAlertLevel,
    pub outstanding_tax: Balance,
    pub due_at: Timestamp,
    pub triggered_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct OptimizationPlan {
    pub estimated_savings: Balance,
    pub recommended_installments: u8,
    pub should_prepay: bool,
    pub review_exemption: bool,
    pub supporting_reference: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum AuditAction {
    JurisdictionProfileConfigured,
    RuleConfigured,
    AssessmentUpdated,
    TaxCalculated,
    TaxPaid,
    PaymentReceiptGenerated,
    ReportingSubmitted,
    LegalDocumentUpdated,
    ComplianceChecked,
    ComplianceViolation,
    MonitoringAlertRaised,
    OptimizationReviewed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct AuditEntry {
    pub action: AuditAction,
    pub property_id: u64,
    pub jurisdiction_code: u32,
    pub reporting_period: u64,
    pub actor: AccountId,
    pub timestamp: Timestamp,
    pub amount: Balance,
    pub reference_hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ComplianceSnapshot {
    pub property_id: u64,
    pub jurisdiction_code: u32,
    pub reporting_period: u64,
    pub registry_compliant: bool,
    pub tax_current: bool,
    pub outstanding_tax: Balance,
    pub reporting_submitted: bool,
    pub legal_documents_verified: bool,
    pub active_alerts: u32,
    pub status: TaxStatus,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Unauthorized,
    RuleNotFound,
    AssessmentNotFound,
    RecordNotFound,
    DocumentNotFound,
    InactiveRule,
    InvalidRate,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unauthorized => write!(f, "Caller is not authorized"),
            Self::RuleNotFound => write!(f, "Tax rule not found"),
            Self::AssessmentNotFound => write!(f, "Property assessment not found"),
            Self::RecordNotFound => write!(f, "Tax record not found"),
            Self::DocumentNotFound => write!(f, "Legal document not found"),
            Self::InactiveRule => write!(f, "Tax rule is inactive"),
            Self::InvalidRate => write!(f, "Tax configuration is invalid"),
        }
    }
}

impl ContractError for Error {
    fn error_code(&self) -> u32 {
        match self {
            Self::Unauthorized => {
                propchain_traits::errors::compliance_codes::COMPLIANCE_UNAUTHORIZED
            }
            Self::RuleNotFound
            | Self::AssessmentNotFound
            | Self::RecordNotFound
            | Self::DocumentNotFound
            | Self::InactiveRule
            | Self::InvalidRate => {
                propchain_traits::errors::compliance_codes::COMPLIANCE_CHECK_FAILED
            }
        }
    }

    fn error_description(&self) -> &'static str {
        match self {
            Self::Unauthorized => "Caller does not have permission to manage tax compliance state",
            Self::RuleNotFound => "No tax rule was configured for the requested jurisdiction",
            Self::AssessmentNotFound => {
                "No property assessment is available for the requested jurisdiction"
            }
            Self::RecordNotFound => "No tax record exists for the requested reporting period",
            Self::DocumentNotFound => "The requested legal document was not registered",
            Self::InactiveRule => "The tax rule for the requested jurisdiction is inactive",
            Self::InvalidRate => {
                "The configured tax or profile rate exceeds the deterministic bounds"
            }
        }
    }

    fn error_category(&self) -> ErrorCategory {
        ErrorCategory::Compliance
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[ink::contract]
mod tax_compliance {
    use super::*;

    #[ink(event)]
    pub struct JurisdictionProfileConfigured {
        #[ink(topic)]
        jurisdiction_code: u32,
        authority_hash: [u8; 32],
    }

    #[ink(event)]
    pub struct TaxCalculated {
        #[ink(topic)]
        property_id: u64,
        #[ink(topic)]
        jurisdiction_code: u32,
        reporting_period: u64,
        tax_due: Balance,
    }

    #[ink(event)]
    pub struct TaxPaid {
        #[ink(topic)]
        property_id: u64,
        #[ink(topic)]
        jurisdiction_code: u32,
        reporting_period: u64,
        amount: Balance,
        outstanding_tax: Balance,
    }

    #[ink(event)]
    pub struct ComplianceViolation {
        #[ink(topic)]
        property_id: u64,
        #[ink(topic)]
        jurisdiction_code: u32,
        reporting_period: u64,
        outstanding_tax: Balance,
        registry_compliant: bool,
    }

    #[ink(event)]
    pub struct ReportingHookTriggered {
        #[ink(topic)]
        property_id: u64,
        #[ink(topic)]
        jurisdiction_code: u32,
        reporting_period: u64,
        report_hash: [u8; 32],
    }

    #[ink(event)]
    pub struct LegalDocumentHookTriggered {
        #[ink(topic)]
        property_id: u64,
        #[ink(topic)]
        jurisdiction_code: u32,
        document_hash: [u8; 32],
        verified: bool,
    }

    #[ink(event)]
    pub struct ComplianceRegistrySyncRequested {
        #[ink(topic)]
        property_id: u64,
        #[ink(topic)]
        jurisdiction_code: u32,
        reporting_period: u64,
        outstanding_tax: Balance,
        legal_documents_verified: bool,
        reporting_submitted: bool,
    }

    #[ink(storage)]
    pub struct TaxComplianceModule {
        admin: AccountId,
        compliance_registry: Option<AccountId>,
        tax_rules: Mapping<u32, TaxRule>,
        jurisdiction_profiles: Mapping<u32, JurisdictionProfile>,
        property_assessments: Mapping<(u64, u32), PropertyAssessment>,
        tax_records: Mapping<(u64, u32, u64), TaxRecord>,
        payment_receipts: Mapping<(u64, u32, u64), PaymentReceipt>,
        legal_documents: Mapping<(u64, u32, u8), LegalDocumentRecord>,
        latest_reporting_period: Mapping<(u64, u32), u64>,
        audit_logs: Mapping<(u64, u64), AuditEntry>,
        audit_log_count: Mapping<u64, u64>,
        compliance_alerts: Mapping<(u64, u64), ComplianceAlert>,
        compliance_alert_count: Mapping<u64, u64>,
    }

    impl TaxComplianceModule {
        #[ink(constructor)]
        pub fn new(compliance_registry: Option<AccountId>) -> Self {
            Self {
                admin: Self::env().caller(),
                compliance_registry,
                tax_rules: Mapping::default(),
                jurisdiction_profiles: Mapping::default(),
                property_assessments: Mapping::default(),
                tax_records: Mapping::default(),
                payment_receipts: Mapping::default(),
                legal_documents: Mapping::default(),
                latest_reporting_period: Mapping::default(),
                audit_logs: Mapping::default(),
                audit_log_count: Mapping::default(),
                compliance_alerts: Mapping::default(),
                compliance_alert_count: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn set_compliance_registry(&mut self, registry: Option<AccountId>) -> Result<()> {
            self.ensure_admin()?;
            self.compliance_registry = registry;
            Ok(())
        }

        #[ink(message)]
        pub fn configure_jurisdiction_profile(
            &mut self,
            jurisdiction: Jurisdiction,
            profile: JurisdictionProfile,
        ) -> Result<()> {
            self.ensure_admin()?;
            if profile.surcharge_basis_points > BASIS_POINTS_DENOMINATOR as u32
                || profile.early_payment_discount_basis_points > BASIS_POINTS_DENOMINATOR as u32
            {
                return Err(Error::InvalidRate);
            }
            self.jurisdiction_profiles
                .insert(jurisdiction.code, &profile);
            self.log_audit(
                0,
                jurisdiction.code,
                0,
                AuditAction::JurisdictionProfileConfigured,
                0,
                profile.authority_hash,
            );
            self.env().emit_event(JurisdictionProfileConfigured {
                jurisdiction_code: jurisdiction.code,
                authority_hash: profile.authority_hash,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn configure_tax_rule(
            &mut self,
            jurisdiction: Jurisdiction,
            rule: TaxRule,
        ) -> Result<()> {
            self.ensure_admin()?;
            if rule.rate_basis_points > BASIS_POINTS_DENOMINATOR as u32
                || rule.penalty_basis_points > BASIS_POINTS_DENOMINATOR as u32
            {
                return Err(Error::InvalidRate);
            }
            self.tax_rules.insert(jurisdiction.code, &rule);
            self.log_audit(
                0,
                jurisdiction.code,
                0,
                AuditAction::RuleConfigured,
                0,
                [0u8; 32],
            );
            Ok(())
        }

        #[ink(message)]
        pub fn set_property_assessment(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
            owner: AccountId,
            assessed_value: Balance,
            exemption_override: Balance,
        ) -> Result<()> {
            self.ensure_admin()?;
            let assessment = PropertyAssessment {
                owner,
                assessed_value,
                exemption_override,
                last_assessed_at: self.env().block_timestamp(),
                legal_documents_verified: false,
                reporting_submitted: false,
            };
            self.property_assessments
                .insert((property_id, jurisdiction.code), &assessment);
            self.log_audit(
                property_id,
                jurisdiction.code,
                0,
                AuditAction::AssessmentUpdated,
                assessed_value,
                [0u8; 32],
            );
            Ok(())
        }

        #[ink(message)]
        pub fn calculate_tax(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
        ) -> Result<TaxRecord> {
            self.ensure_admin()?;
            let now = self.env().block_timestamp();
            let rule = self.get_active_rule(jurisdiction.code)?;
            let assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            let profile = self.jurisdiction_profiles.get(jurisdiction.code);
            let reporting_period = self.reporting_period(now, rule.reporting_frequency);
            let existing = self
                .tax_records
                .get((property_id, jurisdiction.code, reporting_period));
            let (record, breakdown) = tax_engine::calculate_tax_record(
                property_id,
                jurisdiction.code,
                rule,
                profile,
                assessment,
                existing,
                now,
            );
            self.tax_records
                .insert((property_id, jurisdiction.code, reporting_period), &record);
            self.latest_reporting_period
                .insert((property_id, jurisdiction.code), &reporting_period);

            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::TaxCalculated,
                breakdown.total_due,
                [0u8; 32],
            );
            self.env().emit_event(TaxCalculated {
                property_id,
                jurisdiction_code: jurisdiction.code,
                reporting_period,
                tax_due: record.tax_due,
            });

            let alerts = compliance::generate_alerts(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                Some(record),
                self.registry_compliant(assessment.owner),
                now,
            );
            let snapshot = compliance::build_snapshot(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                Some(record),
                self.registry_compliant(assessment.owner),
                alerts.len() as u32,
            );
            self.emit_registry_sync_requested(snapshot);

            Ok(record)
        }

        #[ink(message)]
        pub fn calculate_tax_breakdown(
            &self,
            property_id: u64,
            jurisdiction_code: u32,
            reporting_period: u64,
        ) -> Option<TaxBreakdown> {
            let rule = self.tax_rules.get(jurisdiction_code)?;
            let assessment = self
                .property_assessments
                .get((property_id, jurisdiction_code))?;
            let profile = self.jurisdiction_profiles.get(jurisdiction_code);
            let record =
                self.tax_records
                    .get((property_id, jurisdiction_code, reporting_period))?;
            Some(tax_engine::build_breakdown(
                rule,
                profile,
                assessment,
                record,
                self.env().block_timestamp(),
            ))
        }

        #[ink(message)]
        pub fn record_tax_payment(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
            reporting_period: u64,
            amount: Balance,
            payment_reference: [u8; 32],
        ) -> Result<TaxRecord> {
            self.ensure_admin()?;
            let now = self.env().block_timestamp();
            let rule = self.get_active_rule(jurisdiction.code)?;
            let assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            let mut record = self
                .tax_records
                .get((property_id, jurisdiction.code, reporting_period))
                .ok_or(Error::RecordNotFound)?;
            let (updated_record, receipt) =
                payments::apply_payment(record, amount, payment_reference, now);
            record = updated_record;
            self.tax_records
                .insert((property_id, jurisdiction.code, reporting_period), &record);
            self.payment_receipts
                .insert((property_id, jurisdiction.code, reporting_period), &receipt);

            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::TaxPaid,
                amount,
                payment_reference,
            );
            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::PaymentReceiptGenerated,
                receipt.outstanding_balance,
                payment_reference,
            );
            self.env().emit_event(TaxPaid {
                property_id,
                jurisdiction_code: jurisdiction.code,
                reporting_period,
                amount,
                outstanding_tax: payments::outstanding_tax(&record),
            });

            let snapshot = compliance::build_snapshot(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                Some(record),
                self.registry_compliant(assessment.owner),
                0,
            );
            self.emit_registry_sync_requested(snapshot);

            Ok(record)
        }

        #[ink(message)]
        pub fn get_payment_receipt(
            &self,
            property_id: u64,
            jurisdiction_code: u32,
            reporting_period: u64,
        ) -> Option<PaymentReceipt> {
            self.payment_receipts
                .get((property_id, jurisdiction_code, reporting_period))
        }

        #[ink(message)]
        pub fn record_reporting_submission(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
            reporting_period: u64,
            report_hash: [u8; 32],
        ) -> Result<()> {
            self.ensure_admin()?;
            let rule = self.get_active_rule(jurisdiction.code)?;
            let mut assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            assessment.reporting_submitted = true;
            self.property_assessments
                .insert((property_id, jurisdiction.code), &assessment);

            let mut record = self
                .tax_records
                .get((property_id, jurisdiction.code, reporting_period))
                .ok_or(Error::RecordNotFound)?;
            record.report_hash = report_hash;
            self.tax_records
                .insert((property_id, jurisdiction.code, reporting_period), &record);

            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::ReportingSubmitted,
                0,
                report_hash,
            );
            self.env().emit_event(ReportingHookTriggered {
                property_id,
                jurisdiction_code: jurisdiction.code,
                reporting_period,
                report_hash,
            });

            let snapshot = compliance::build_snapshot(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                Some(record),
                self.registry_compliant(assessment.owner),
                0,
            );
            self.emit_registry_sync_requested(snapshot);

            Ok(())
        }

        #[ink(message)]
        pub fn record_legal_document(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
            document_hash: [u8; 32],
            verified: bool,
        ) -> Result<()> {
            self.upsert_legal_document(
                property_id,
                jurisdiction,
                LegalDocumentType::TitleDeed,
                document_hash,
                self.env().block_timestamp(),
                0,
                verified,
            )
        }

        #[ink(message)]
        pub fn upsert_legal_document(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
            document_type: LegalDocumentType,
            document_hash: [u8; 32],
            issued_at: Timestamp,
            expires_at: Timestamp,
            verified: bool,
        ) -> Result<()> {
            self.ensure_admin()?;
            let rule = self.get_active_rule(jurisdiction.code)?;
            let mut assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            let record = legal::build_document_record(
                property_id,
                jurisdiction.code,
                document_type,
                document_hash,
                issued_at,
                expires_at,
                verified,
                self.env().block_timestamp(),
            );
            self.legal_documents.insert(
                (property_id, jurisdiction.code, document_type.key()),
                &record,
            );
            assessment.legal_documents_verified = legal::assessment_verified(&record);
            self.property_assessments
                .insert((property_id, jurisdiction.code), &assessment);

            let reporting_period = self
                .latest_reporting_period
                .get((property_id, jurisdiction.code))
                .unwrap_or(0);
            let tax_record =
                self.tax_records
                    .get((property_id, jurisdiction.code, reporting_period));

            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::LegalDocumentUpdated,
                0,
                document_hash,
            );
            self.env().emit_event(LegalDocumentHookTriggered {
                property_id,
                jurisdiction_code: jurisdiction.code,
                document_hash,
                verified,
            });

            let snapshot = compliance::build_snapshot(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                tax_record,
                self.registry_compliant(assessment.owner),
                0,
            );
            self.emit_registry_sync_requested(snapshot);

            Ok(())
        }

        #[ink(message)]
        pub fn get_legal_document(
            &self,
            property_id: u64,
            jurisdiction_code: u32,
            document_type: LegalDocumentType,
        ) -> Option<LegalDocumentRecord> {
            self.legal_documents
                .get((property_id, jurisdiction_code, document_type.key()))
        }

        #[ink(message)]
        pub fn monitor_compliance(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
        ) -> Result<Vec<ComplianceAlert>> {
            let assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            let rule = self.get_active_rule(jurisdiction.code)?;
            let reporting_period = self
                .latest_reporting_period
                .get((property_id, jurisdiction.code))
                .unwrap_or(
                    self.reporting_period(self.env().block_timestamp(), rule.reporting_frequency),
                );
            let record = self
                .tax_records
                .get((property_id, jurisdiction.code, reporting_period));
            let alerts = compliance::generate_alerts(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                record,
                self.registry_compliant(assessment.owner),
                self.env().block_timestamp(),
            );

            for alert in &alerts {
                self.store_alert(*alert);
                self.log_audit(
                    property_id,
                    jurisdiction.code,
                    alert.reporting_period,
                    AuditAction::MonitoringAlertRaised,
                    alert.outstanding_tax,
                    [0u8; 32],
                );
            }

            Ok(alerts)
        }

        #[ink(message)]
        pub fn recommend_tax_optimization(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
        ) -> Result<OptimizationPlan> {
            let rule = self.get_active_rule(jurisdiction.code)?;
            let assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            let reporting_period = self
                .latest_reporting_period
                .get((property_id, jurisdiction.code))
                .unwrap_or(
                    self.reporting_period(self.env().block_timestamp(), rule.reporting_frequency),
                );
            let record = self
                .tax_records
                .get((property_id, jurisdiction.code, reporting_period));
            let profile = self.jurisdiction_profiles.get(jurisdiction.code);
            let plan = optimization::recommend_plan(
                rule,
                profile,
                assessment,
                record,
                self.env().block_timestamp(),
            );
            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::OptimizationReviewed,
                plan.estimated_savings,
                plan.supporting_reference,
            );
            Ok(plan)
        }

        #[ink(message)]
        pub fn check_compliance(
            &mut self,
            property_id: u64,
            jurisdiction: Jurisdiction,
        ) -> Result<ComplianceSnapshot> {
            let assessment = self
                .property_assessments
                .get((property_id, jurisdiction.code))
                .ok_or(Error::AssessmentNotFound)?;
            let rule = self.get_active_rule(jurisdiction.code)?;
            let reporting_period = self
                .latest_reporting_period
                .get((property_id, jurisdiction.code))
                .unwrap_or(
                    self.reporting_period(self.env().block_timestamp(), rule.reporting_frequency),
                );
            let record = self
                .tax_records
                .get((property_id, jurisdiction.code, reporting_period));
            let alerts = compliance::generate_alerts(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                record,
                self.registry_compliant(assessment.owner),
                self.env().block_timestamp(),
            );
            let snapshot = compliance::build_snapshot(
                property_id,
                jurisdiction.code,
                rule,
                assessment,
                record,
                self.registry_compliant(assessment.owner),
                alerts.len() as u32,
            );

            self.log_audit(
                property_id,
                jurisdiction.code,
                reporting_period,
                AuditAction::ComplianceChecked,
                snapshot.outstanding_tax,
                [0u8; 32],
            );

            if !snapshot.tax_current || !snapshot.registry_compliant {
                self.log_audit(
                    property_id,
                    jurisdiction.code,
                    reporting_period,
                    AuditAction::ComplianceViolation,
                    snapshot.outstanding_tax,
                    [0u8; 32],
                );
                self.env().emit_event(ComplianceViolation {
                    property_id,
                    jurisdiction_code: jurisdiction.code,
                    reporting_period,
                    outstanding_tax: snapshot.outstanding_tax,
                    registry_compliant: snapshot.registry_compliant,
                });
            }

            Ok(snapshot)
        }

        #[ink(message)]
        pub fn get_tax_rule(&self, jurisdiction_code: u32) -> Option<TaxRule> {
            self.tax_rules.get(jurisdiction_code)
        }

        #[ink(message)]
        pub fn get_jurisdiction_profile(
            &self,
            jurisdiction_code: u32,
        ) -> Option<JurisdictionProfile> {
            self.jurisdiction_profiles.get(jurisdiction_code)
        }

        #[ink(message)]
        pub fn get_property_assessment(
            &self,
            property_id: u64,
            jurisdiction_code: u32,
        ) -> Option<PropertyAssessment> {
            self.property_assessments
                .get((property_id, jurisdiction_code))
        }

        #[ink(message)]
        pub fn get_tax_record(
            &self,
            property_id: u64,
            jurisdiction_code: u32,
            reporting_period: u64,
        ) -> Option<TaxRecord> {
            self.tax_records
                .get((property_id, jurisdiction_code, reporting_period))
        }

        #[ink(message)]
        pub fn get_audit_trail(&self, property_id: u64, limit: u64) -> Vec<AuditEntry> {
            let count = self.audit_log_count.get(property_id).unwrap_or(0);
            let start = count.saturating_sub(limit);
            let mut entries = Vec::new();
            for index in start..count {
                if let Some(entry) = self.audit_logs.get((property_id, index)) {
                    entries.push(entry);
                }
            }
            entries
        }

        #[ink(message)]
        pub fn get_compliance_alerts(&self, property_id: u64, limit: u64) -> Vec<ComplianceAlert> {
            let count = self.compliance_alert_count.get(property_id).unwrap_or(0);
            let start = count.saturating_sub(limit);
            let mut entries = Vec::new();
            for index in start..count {
                if let Some(entry) = self.compliance_alerts.get((property_id, index)) {
                    entries.push(entry);
                }
            }
            entries
        }

        fn ensure_admin(&self) -> Result<()> {
            if self.env().caller() != self.admin {
                return Err(Error::Unauthorized);
            }
            Ok(())
        }

        fn get_active_rule(&self, jurisdiction_code: u32) -> Result<TaxRule> {
            match self.tax_rules.get(jurisdiction_code) {
                Some(rule) if rule.active => Ok(rule),
                Some(_) => Err(Error::InactiveRule),
                None => Err(Error::RuleNotFound),
            }
        }

        fn reporting_period(&self, now: Timestamp, frequency: ReportingFrequency) -> u64 {
            now / frequency.period_millis()
        }

        fn registry_compliant(&self, owner: AccountId) -> bool {
            match self.compliance_registry {
                Some(registry) => {
                    use ink::env::call::FromAccountId;
                    let checker: ink::contract_ref!(ComplianceChecker) =
                        FromAccountId::from_account_id(registry);
                    checker.is_compliant(owner)
                }
                None => true,
            }
        }

        fn emit_registry_sync_requested(&self, snapshot: ComplianceSnapshot) {
            self.env().emit_event(ComplianceRegistrySyncRequested {
                property_id: snapshot.property_id,
                jurisdiction_code: snapshot.jurisdiction_code,
                reporting_period: snapshot.reporting_period,
                outstanding_tax: snapshot.outstanding_tax,
                legal_documents_verified: snapshot.legal_documents_verified,
                reporting_submitted: snapshot.reporting_submitted,
            });
        }

        fn store_alert(&mut self, alert: ComplianceAlert) {
            let count = self
                .compliance_alert_count
                .get(alert.property_id)
                .unwrap_or(0);
            self.compliance_alerts
                .insert((alert.property_id, count), &alert);
            self.compliance_alert_count
                .insert(alert.property_id, &(count + 1));
        }

        fn log_audit(
            &mut self,
            property_id: u64,
            jurisdiction_code: u32,
            reporting_period: u64,
            action: AuditAction,
            amount: Balance,
            reference_hash: [u8; 32],
        ) {
            let count = self.audit_log_count.get(property_id).unwrap_or(0);
            let entry = AuditEntry {
                action,
                property_id,
                jurisdiction_code,
                reporting_period,
                actor: self.env().caller(),
                timestamp: self.env().block_timestamp(),
                amount,
                reference_hash,
            };
            self.audit_logs.insert((property_id, count), &entry);
            self.audit_log_count.insert(property_id, &(count + 1));
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;
        use ink::env::DefaultEnvironment;

        fn jurisdiction() -> Jurisdiction {
            Jurisdiction {
                code: 1001,
                country_code: *b"US",
                region_code: 12,
                locality_code: 34,
            }
        }

        fn profile() -> JurisdictionProfile {
            JurisdictionProfile {
                surcharge_basis_points: 50,
                early_payment_discount_basis_points: 100,
                late_payment_grace_period: 5 * 24 * 60 * 60 * 1_000,
                optimization_window: 15 * 24 * 60 * 60 * 1_000,
                requires_digital_stamp: true,
                authority_hash: [3u8; 32],
            }
        }

        fn rule() -> TaxRule {
            TaxRule {
                rate_basis_points: 250,
                fixed_charge: 1_000,
                exemption_amount: 10_000,
                payment_due_period: 30 * 24 * 60 * 60 * 1_000,
                reporting_frequency: ReportingFrequency::Annual,
                penalty_basis_points: 500,
                requires_reporting: true,
                requires_legal_documents: true,
                active: true,
            }
        }

        #[ink::test]
        fn calculate_tax_uses_jurisdiction_rule() {
            let mut contract = TaxComplianceModule::new(None);
            let owner = AccountId::from([0x02; 32]);

            contract
                .configure_jurisdiction_profile(jurisdiction(), profile())
                .expect("profile");
            contract
                .configure_tax_rule(jurisdiction(), rule())
                .expect("rule");
            contract
                .set_property_assessment(7, jurisdiction(), owner, 200_000, 5_000)
                .expect("assessment");

            let record = contract.calculate_tax(7, jurisdiction()).expect("tax");
            assert_eq!(record.taxable_value, 185_000);
            assert_eq!(record.discount_amount, 46);
            assert_eq!(record.tax_due, 5_602);
            assert_eq!(record.status, TaxStatus::Assessed);
        }

        #[ink::test]
        fn compliance_requires_payment_reporting_and_documents() {
            let mut contract = TaxComplianceModule::new(None);
            let owner = AccountId::from([0x03; 32]);

            contract
                .configure_tax_rule(jurisdiction(), rule())
                .expect("rule");
            contract
                .set_property_assessment(8, jurisdiction(), owner, 120_000, 0)
                .expect("assessment");

            let record = contract.calculate_tax(8, jurisdiction()).expect("tax");
            let initial = contract
                .check_compliance(8, jurisdiction())
                .expect("compliance");
            assert!(!initial.tax_current);
            assert_eq!(initial.outstanding_tax, record.tax_due);

            contract
                .record_tax_payment(
                    8,
                    jurisdiction(),
                    record.reporting_period,
                    record.tax_due,
                    [9u8; 32],
                )
                .expect("payment");
            contract
                .record_reporting_submission(8, jurisdiction(), record.reporting_period, [7u8; 32])
                .expect("report");
            contract
                .upsert_legal_document(
                    8,
                    jurisdiction(),
                    LegalDocumentType::TitleDeed,
                    [8u8; 32],
                    1,
                    10_000,
                    true,
                )
                .expect("document");

            let final_snapshot = contract
                .check_compliance(8, jurisdiction())
                .expect("compliance after hooks");
            assert!(final_snapshot.tax_current);
            assert_eq!(final_snapshot.outstanding_tax, 0);
            assert!(final_snapshot.reporting_submitted);
            assert!(final_snapshot.legal_documents_verified);
        }

        #[ink::test]
        fn monitoring_and_optimization_generate_actionable_outputs() {
            let mut contract = TaxComplianceModule::new(None);
            let owner = AccountId::from([0x04; 32]);

            contract
                .configure_jurisdiction_profile(jurisdiction(), profile())
                .expect("profile");
            contract
                .configure_tax_rule(jurisdiction(), rule())
                .expect("rule");
            contract
                .set_property_assessment(9, jurisdiction(), owner, 100_000, 0)
                .expect("assessment");

            let record = contract.calculate_tax(9, jurisdiction()).expect("tax");
            let alerts = contract
                .monitor_compliance(9, jurisdiction())
                .expect("alerts");
            let plan = contract
                .recommend_tax_optimization(9, jurisdiction())
                .expect("plan");

            assert!(!alerts.is_empty());
            assert!(alerts.iter().any(|alert| matches!(
                alert.alert_type,
                ComplianceAlertType::ReportingMissing | ComplianceAlertType::LegalDocumentsMissing
            )));
            assert!(plan.estimated_savings > 0);
            assert!(plan.should_prepay);

            let receipt_before =
                contract.get_payment_receipt(9, jurisdiction().code, record.reporting_period);
            assert!(receipt_before.is_none());
        }

        #[ink::test]
        fn payment_receipt_and_audit_trail_capture_tax_lifecycle() {
            let mut contract = TaxComplianceModule::new(None);
            let owner = AccountId::from([0x05; 32]);

            contract
                .configure_tax_rule(jurisdiction(), rule())
                .expect("rule");
            contract
                .set_property_assessment(10, jurisdiction(), owner, 100_000, 0)
                .expect("assessment");
            let record = contract.calculate_tax(10, jurisdiction()).expect("tax");
            contract
                .record_tax_payment(
                    10,
                    jurisdiction(),
                    record.reporting_period,
                    record.tax_due / 2,
                    [5u8; 32],
                )
                .expect("payment");

            let receipt = contract
                .get_payment_receipt(10, jurisdiction().code, record.reporting_period)
                .expect("receipt");
            let logs = contract.get_audit_trail(10, 10);
            assert_eq!(receipt.amount_paid, record.tax_due / 2);
            assert_eq!(logs.len(), 4);
            assert_eq!(logs[0].action, AuditAction::AssessmentUpdated);
            assert_eq!(logs[1].action, AuditAction::TaxCalculated);
            assert_eq!(logs[2].action, AuditAction::TaxPaid);
            assert_eq!(logs[3].action, AuditAction::PaymentReceiptGenerated);
        }

        #[ink::test]
        fn legal_document_status_transitions_to_expired() {
            let mut contract = TaxComplianceModule::new(None);
            let owner = AccountId::from([0x06; 32]);
            test::set_block_timestamp::<DefaultEnvironment>(5_000);

            contract
                .configure_tax_rule(jurisdiction(), rule())
                .expect("rule");
            contract
                .set_property_assessment(11, jurisdiction(), owner, 80_000, 0)
                .expect("assessment");
            contract
                .upsert_legal_document(
                    11,
                    jurisdiction(),
                    LegalDocumentType::EnvironmentalPermit,
                    [4u8; 32],
                    1_000,
                    4_000,
                    true,
                )
                .expect("document");

            let document = contract
                .get_legal_document(
                    11,
                    jurisdiction().code,
                    LegalDocumentType::EnvironmentalPermit,
                )
                .expect("stored");
            assert_eq!(document.status, LegalDocumentStatus::Expired);
            assert!(
                !contract
                    .get_property_assessment(11, jurisdiction().code)
                    .expect("assessment")
                    .legal_documents_verified
            );
        }
    }
}
