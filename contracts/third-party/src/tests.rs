// Unit tests for the third-party contract (Issue #101 - extracted from lib.rs)

#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn service_registration_works() {
        let mut contract = ThirdPartyIntegration::new();
        let provider = AccountId::from([0x01; 32]);

        let result = contract.register_service(
            ServiceType::KycProvider,
            String::from("Test KYC"),
            provider,
            String::from("https://api.testkyc.com"),
            String::from("v1"),
            0,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let service = contract.get_service_config(1).unwrap();
        assert_eq!(service.name, "Test KYC");
        assert_eq!(service.service_type, ServiceType::KycProvider);
    }

    #[ink::test]
    fn kyc_flow_works() {
        let mut contract = ThirdPartyIntegration::new();
        let caller = contract.admin;

        contract
            .register_service(
                ServiceType::KycProvider,
                String::from("Test KYC"),
                caller,
                String::from("https://api.testkyc.com"),
                String::from("v1"),
                0,
            )
            .unwrap();

        let request_id = contract
            .initiate_kyc_request(1, caller, String::from("UID123"))
            .unwrap();

        let result = contract.update_kyc_status(
            request_id,
            RequestStatus::Approved,
            2,
            365,
        );
        assert!(result.is_ok());

        assert!(contract.is_kyc_verified(caller, 1));
        assert!(contract.is_kyc_verified(caller, 2));
        assert!(!contract.is_kyc_verified(caller, 3));
    }

    #[ink::test]
    fn payment_flow_works() {
        let mut contract = ThirdPartyIntegration::new();
        let caller = contract.admin;

        contract
            .register_service(
                ServiceType::PaymentGateway,
                String::from("PayGate"),
                caller,
                String::from("https://api.paygate.com"),
                String::from("v1"),
                0,
            )
            .unwrap();

        let target = AccountId::from([0x02; 32]);
        let req_id = contract
            .initiate_fiat_payment(
                1,
                target,
                1,
                10000,
                String::from("USD"),
                String::from("REF123"),
            )
            .unwrap();

        let req1 = contract.get_payment_request(req_id).unwrap();
        assert_eq!(req1.status, RequestStatus::Pending);

        let result = contract.complete_payment(req_id, true, 50000);
        assert!(result.is_ok());

        let req2 = contract.get_payment_request(req_id).unwrap();
        assert_eq!(req2.status, RequestStatus::Approved);
        assert_eq!(req2.equivalent_tokens, 50000);
    }
}
