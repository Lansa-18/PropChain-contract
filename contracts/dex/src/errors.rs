// Error types for the DEX contract (Issue #101 - extracted from lib.rs)

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Unauthorized,
    InvalidPair,
    PoolNotFound,
    InsufficientLiquidity,
    SlippageExceeded,
    OrderNotFound,
    InvalidOrder,
    OrderNotExecutable,
    RewardUnavailable,
    ProposalNotFound,
    ProposalClosed,
    AlreadyVoted,
    InvalidBridgeRoute,
    CrossChainTradeNotFound,
    InsufficientGovernanceBalance,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Unauthorized => write!(f, "Caller is not authorized"),
            Error::InvalidPair => write!(f, "Invalid trading pair"),
            Error::PoolNotFound => write!(f, "Liquidity pool not found"),
            Error::InsufficientLiquidity => write!(f, "Insufficient liquidity"),
            Error::SlippageExceeded => write!(f, "Slippage tolerance exceeded"),
            Error::OrderNotFound => write!(f, "Order not found"),
            Error::InvalidOrder => write!(f, "Invalid order parameters"),
            Error::OrderNotExecutable => write!(f, "Order is not executable"),
            Error::RewardUnavailable => write!(f, "Reward unavailable"),
            Error::ProposalNotFound => write!(f, "Governance proposal not found"),
            Error::ProposalClosed => write!(f, "Governance proposal is closed"),
            Error::AlreadyVoted => write!(f, "Vote already recorded"),
            Error::InvalidBridgeRoute => write!(f, "Invalid cross-chain bridge route"),
            Error::CrossChainTradeNotFound => write!(f, "Cross-chain trade not found"),
            Error::InsufficientGovernanceBalance => {
                write!(f, "Insufficient governance balance")
            }
        }
    }
}

impl ContractError for Error {
    fn error_code(&self) -> u32 {
        match self {
            Error::Unauthorized => dex_codes::DEX_UNAUTHORIZED,
            Error::InvalidPair => dex_codes::DEX_INVALID_PAIR,
            Error::PoolNotFound => dex_codes::DEX_POOL_NOT_FOUND,
            Error::InsufficientLiquidity => dex_codes::DEX_INSUFFICIENT_LIQUIDITY,
            Error::SlippageExceeded => dex_codes::DEX_SLIPPAGE_EXCEEDED,
            Error::OrderNotFound => dex_codes::DEX_ORDER_NOT_FOUND,
            Error::InvalidOrder => dex_codes::DEX_INVALID_ORDER,
            Error::OrderNotExecutable => dex_codes::DEX_ORDER_NOT_EXECUTABLE,
            Error::RewardUnavailable => dex_codes::DEX_REWARD_UNAVAILABLE,
            Error::ProposalNotFound => dex_codes::DEX_PROPOSAL_NOT_FOUND,
            Error::ProposalClosed => dex_codes::DEX_PROPOSAL_CLOSED,
            Error::AlreadyVoted => dex_codes::DEX_ALREADY_VOTED,
            Error::InvalidBridgeRoute => dex_codes::DEX_INVALID_BRIDGE_ROUTE,
            Error::CrossChainTradeNotFound => dex_codes::DEX_CROSS_CHAIN_TRADE_NOT_FOUND,
            Error::InsufficientGovernanceBalance => {
                dex_codes::DEX_INSUFFICIENT_GOVERNANCE_BALANCE
            }
        }
    }

    fn error_description(&self) -> &'static str {
        match self {
            Error::Unauthorized => "Caller does not have permission to perform this operation",
            Error::InvalidPair => "The requested trading pair is invalid or inactive",
            Error::PoolNotFound => "The referenced liquidity pool does not exist",
            Error::InsufficientLiquidity => "Not enough liquidity is available",
            Error::SlippageExceeded => "Trade output is below the allowed threshold",
            Error::OrderNotFound => "The order does not exist",
            Error::InvalidOrder => "Order parameters are invalid",
            Error::OrderNotExecutable => "Order conditions are not satisfied",
            Error::RewardUnavailable => "There are no rewards available to claim",
            Error::ProposalNotFound => "The governance proposal does not exist",
            Error::ProposalClosed => "The governance proposal can no longer be modified",
            Error::AlreadyVoted => "The account has already voted on this proposal",
            Error::InvalidBridgeRoute => "The selected bridge route is not supported",
            Error::CrossChainTradeNotFound => "The cross-chain trade does not exist",
            Error::InsufficientGovernanceBalance => {
                "The account does not hold enough governance tokens"
            }
        }
    }

    fn error_category(&self) -> ErrorCategory {
        ErrorCategory::Dex
    }
}
