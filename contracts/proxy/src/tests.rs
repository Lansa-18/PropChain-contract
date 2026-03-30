// Unit tests for the proxy contract (Issue #101 - extracted from lib.rs)

#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn new_initializes_correctly() {
        let hash = Hash::from([0x42; 32]);
        let proxy = TransparentProxy::new(hash);
        assert_eq!(proxy.code_hash(), hash);
        assert_eq!(proxy.current_version(), (1, 0, 0));
        assert_eq!(proxy.get_version_history().len(), 1);
        assert_eq!(proxy.migration_state(), MigrationState::None);
        assert!(!proxy.is_paused());
    }

    #[ink::test]
    fn propose_upgrade_works() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);

        let new_hash = Hash::from([0x43; 32]);
        let result = proxy.propose_upgrade(
            new_hash,
            1,
            1,
            0,
            String::from("Feature upgrade"),
            String::from("No migration needed"),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let proposal = proxy.get_proposal(1).unwrap();
        assert_eq!(proposal.new_code_hash, new_hash);
        assert!(!proposal.cancelled);
        assert!(!proposal.executed);
    }

    #[ink::test]
    fn version_compatibility_check_works() {
        let hash = Hash::from([0x42; 32]);
        let proxy = TransparentProxy::new(hash);

        assert!(proxy.check_compatibility(1, 1, 0));
        assert!(proxy.check_compatibility(2, 0, 0));
        assert!(!proxy.check_compatibility(0, 9, 0));
        assert!(!proxy.check_compatibility(1, 0, 0));
    }

    #[ink::test]
    fn direct_upgrade_works() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);

        let new_hash = Hash::from([0x43; 32]);
        let result = proxy.upgrade_to(new_hash);
        assert!(result.is_ok());
        assert_eq!(proxy.code_hash(), new_hash);
        assert_eq!(proxy.get_version_history().len(), 2);
    }

    #[ink::test]
    fn rollback_works() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);

        let new_hash = Hash::from([0x43; 32]);
        proxy.upgrade_to(new_hash).unwrap();
        assert_eq!(proxy.code_hash(), new_hash);

        let rollback_result = proxy.rollback();
        assert!(rollback_result.is_ok());
        assert_eq!(proxy.code_hash(), hash);
        assert_eq!(proxy.migration_state(), MigrationState::RolledBack);
    }

    #[ink::test]
    fn rollback_fails_with_no_history() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);
        assert_eq!(proxy.rollback(), Err(Error::NoPreviousVersion));
    }

    #[ink::test]
    fn emergency_pause_works() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);
        assert!(!proxy.is_paused());

        proxy.toggle_emergency_pause().unwrap();
        assert!(proxy.is_paused());

        let new_hash = Hash::from([0x43; 32]);
        assert_eq!(proxy.upgrade_to(new_hash), Err(Error::EmergencyPauseActive));

        proxy.toggle_emergency_pause().unwrap();
        assert!(!proxy.is_paused());
    }

    #[ink::test]
    fn cancel_upgrade_works() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);

        let new_hash = Hash::from([0x43; 32]);
        proxy
            .propose_upgrade(new_hash, 1, 1, 0, String::from("Test"), String::from(""))
            .unwrap();

        let result = proxy.cancel_upgrade(1);
        assert!(result.is_ok());

        let proposal = proxy.get_proposal(1).unwrap();
        assert!(proposal.cancelled);
    }

    #[ink::test]
    fn governor_management_works() {
        let hash = Hash::from([0x42; 32]);
        let mut proxy = TransparentProxy::new(hash);

        let new_governor = AccountId::from([0x02; 32]);
        proxy.add_governor(new_governor).unwrap();
        assert_eq!(proxy.governors().len(), 2);

        proxy.remove_governor(new_governor).unwrap();
        assert_eq!(proxy.governors().len(), 1);
    }
}
