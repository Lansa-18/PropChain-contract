// Unit tests for the staking contract (Issue #101 - extracted from lib.rs)

#[cfg(test)]
mod tests {
    use super::*;

    fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
    }

    fn set_caller(caller: AccountId) {
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
    }

    fn advance_block(n: u32) {
        for _ in 0..n {
            ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
        }
    }

    fn create_staking() -> Staking {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        Staking::new(500, 1_000)
    }

    #[ink::test]
    fn constructor_sets_defaults() {
        let staking = create_staking();
        let accounts = default_accounts();
        assert_eq!(staking.get_admin(), accounts.alice);
        assert_eq!(staking.get_total_staked(), 0);
        assert_eq!(staking.get_reward_pool(), 0);
        assert_eq!(staking.get_min_stake(), 1_000);
    }

    #[ink::test]
    fn constructor_clamps_zero_min_stake() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let staking = Staking::new(500, 0);
        assert_eq!(staking.get_min_stake(), constants::STAKING_MIN_AMOUNT);
    }

    #[ink::test]
    fn stake_succeeds() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        let result = staking.stake(10_000, LockPeriod::Flexible);
        assert!(result.is_ok());
        assert_eq!(staking.get_total_staked(), 10_000);

        let info = staking.get_stake(accounts.bob).unwrap();
        assert_eq!(info.amount, 10_000);
        assert_eq!(info.lock_period, LockPeriod::Flexible);
    }

    #[ink::test]
    fn stake_below_minimum_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        assert_eq!(
            staking.stake(500, LockPeriod::Flexible),
            Err(Error::InsufficientAmount)
        );
    }

    #[ink::test]
    fn stake_zero_amount_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        assert_eq!(
            staking.stake(0, LockPeriod::Flexible),
            Err(Error::ZeroAmount)
        );
    }

    #[ink::test]
    fn double_stake_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        staking.stake(10_000, LockPeriod::Flexible).unwrap();
        assert_eq!(
            staking.stake(10_000, LockPeriod::Flexible),
            Err(Error::AlreadyStaked)
        );
    }

    #[ink::test]
    fn unstake_flexible_succeeds() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        staking.stake(10_000, LockPeriod::Flexible).unwrap();
        let result = staking.unstake();
        assert!(result.is_ok());
        assert_eq!(staking.get_total_staked(), 0);
        assert!(staking.get_stake(accounts.bob).is_none());
    }

    #[ink::test]
    fn unstake_locked_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        staking.stake(10_000, LockPeriod::ThirtyDays).unwrap();
        assert_eq!(staking.unstake(), Err(Error::LockActive));
    }

    #[ink::test]
    fn unstake_no_stake_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        assert_eq!(staking.unstake(), Err(Error::StakeNotFound));
    }

    #[ink::test]
    fn claim_rewards_with_pool() {
        let mut staking = create_staking();
        let accounts = default_accounts();

        set_caller(accounts.alice);
        staking.fund_reward_pool(1_000_000_000_000).unwrap();

        set_caller(accounts.bob);
        staking
            .stake(1_000_000_000_000_000, LockPeriod::Flexible)
            .unwrap();

        advance_block(100_000);

        let pending = staking.get_pending_rewards(accounts.bob);
        assert!(
            pending > 0,
            "pending rewards should be > 0, got {}",
            pending
        );

        let result = staking.claim_rewards();
        assert!(result.is_ok());
    }

    #[ink::test]
    fn claim_rewards_no_stake_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        assert_eq!(staking.claim_rewards(), Err(Error::StakeNotFound));
    }

    #[ink::test]
    fn delegate_governance_succeeds() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        staking.stake(10_000, LockPeriod::Flexible).unwrap();

        assert_eq!(staking.get_governance_power(accounts.bob), 10_000);

        staking.delegate_governance(accounts.charlie).unwrap();
        assert_eq!(staking.get_governance_power(accounts.bob), 0);
        assert_eq!(staking.get_governance_power(accounts.charlie), 10_000);
    }

    #[ink::test]
    fn self_delegation_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        staking.stake(10_000, LockPeriod::Flexible).unwrap();
        assert_eq!(
            staking.delegate_governance(accounts.bob),
            Err(Error::InvalidDelegate)
        );
    }

    #[ink::test]
    fn fund_pool_non_admin_fails() {
        let mut staking = create_staking();
        let accounts = default_accounts();
        set_caller(accounts.bob);
        assert_eq!(staking.fund_reward_pool(1000), Err(Error::Unauthorized));
    }

    #[ink::test]
    fn update_config_succeeds() {
        let mut staking = create_staking();
        staking.update_config(5_000, 1000).unwrap();
        assert_eq!(staking.get_min_stake(), 5_000);
    }

    #[ink::test]
    fn update_config_zero_min_fails() {
        let mut staking = create_staking();
        assert_eq!(staking.update_config(0, 1000), Err(Error::InvalidConfig));
    }

    #[ink::test]
    fn lock_period_durations_correct() {
        assert_eq!(LockPeriod::Flexible.duration_blocks(), 0);
        assert_eq!(
            LockPeriod::ThirtyDays.duration_blocks(),
            constants::LOCK_PERIOD_30_DAYS
        );
        assert_eq!(
            LockPeriod::NinetyDays.duration_blocks(),
            constants::LOCK_PERIOD_90_DAYS
        );
        assert_eq!(
            LockPeriod::OneYear.duration_blocks(),
            constants::LOCK_PERIOD_1_YEAR
        );
    }

    #[ink::test]
    fn multipliers_increase_with_lock() {
        assert!(LockPeriod::ThirtyDays.multiplier() > LockPeriod::Flexible.multiplier());
        assert!(LockPeriod::NinetyDays.multiplier() > LockPeriod::ThirtyDays.multiplier());
        assert!(LockPeriod::OneYear.multiplier() > LockPeriod::NinetyDays.multiplier());
    }
}
