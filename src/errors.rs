pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The candle data provided is empty. Backtesting requires at least one candle.
    #[error("Candle data is empty: backtesting requires at least one candle")]
    CandleDataEmpty,

    /// The initial or current balance is not positive. Trading requires a positive balance.
    #[error("Balance must be positive (got: {0})")]
    NegZeroBalance(f64),

    /// The wallet does not have enough funds to place the order.
    /// Expected: {0}, Available: {1}
    #[error("Insufficient funds: required {0}, available {1}")]
    InsufficientFunds(f64, f64),

    /// The order was not found.
    #[error("Order not found")]
    OrderNotFound,

    /// The position was not found.
    #[error("Position not found")]
    PositionNotFound,

    /// An unreachable context was encountered. This is likely a bug.
    #[error("Unreachable context (internal error): {0}")]
    Unreachable(String),

    /// I/O error occurred.
    // utils.rs
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization/deserialization error occurred.
    #[cfg(feature = "serde")]
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
