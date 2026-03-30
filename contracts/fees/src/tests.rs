// Unit tests for the fees contract (Issue #101 - extracted from lib.rs)

#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn test_dynamic_fee_calculation() {
        let contract = FeeManager::new(1000, 100, 100_000);
        let fee = contract.calculate_fee(FeeOperation::RegisterProperty);
        assert!((100..=100_000).contains(&fee));
    }

    #[ink::test]
    fn test_premium_auction_flow() {
        let mut contract = FeeManager::new(100, 10, 10_000);
        let auction_id = contract
            .create_premium_auction(1, 500, 3600)
            .expect("create auction");
        assert_eq!(auction_id, 1);
        let auction = contract.get_auction(auction_id).unwrap();
        assert_eq!(auction.property_id, 1);
        assert_eq!(auction.min_bid, 500);
        assert!(!auction.settled);

        assert!(contract.place_bid(auction_id, 600).is_ok());
        let auction = contract.get_auction(auction_id).unwrap();
        assert_eq!(auction.current_bid, 600);
    }

    #[ink::test]
    fn test_fee_report() {
        let contract = FeeManager::new(1000, 100, 50_000);
        let report = contract.get_fee_report();
        assert_eq!(report.total_fees_collected, 0);
        assert!(report.recommended_fee >= 100);
    }

    #[ink::test]
    fn test_fee_estimate_recommendation() {
        let contract = FeeManager::new(1000, 100, 50_000);
        let est = contract.get_fee_estimate(FeeOperation::TransferProperty);
        assert!(!est.recommendation.is_empty());
        assert!(!est.congestion_level.is_empty());
    }
}
