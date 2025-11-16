#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a financial candle (or candlestick) with open, high, low, close, volume, and bid/ask data.
///
/// A candle is a fundamental data structure in financial markets, representing price movements
/// over a specific time period. It includes the opening price, highest price, lowest price,
/// closing price, trading volume, and bid/ask spread information.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    bid: f64,
}

impl From<(f64, f64, f64, f64, f64)> for Candle {
    fn from((open, high, low, close, volume): (f64, f64, f64, f64, f64)) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            bid: 0.0,
        }
    }
}

impl From<(f64, f64, f64, f64, f64, f64)> for Candle {
    fn from((open, high, low, close, volume, bid): (f64, f64, f64, f64, f64, f64)) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            bid,
        }
    }
}

impl Candle {
    /// Returns the opening price of the candle.
    pub fn open(&self) -> f64 {
        self.open
    }

    /// Returns the highest price reached during the candle period.
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Returns the lowest price reached during the candle period.
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Returns the closing price of the candle.
    pub fn close(&self) -> f64 {
        self.close
    }

    /// Returns the trading volume during the candle period.
    pub fn volume(&self) -> f64 {
        self.volume
    }

    /// Returns the bid price of the candle.
    pub fn bid(&self) -> f64 {
        self.bid
    }

    /// Returns the ask price of the candle, calculated as volume minus bid.
    ///
    /// Note: This is a simplified calculation and may not reflect the actual market ask price.
    pub fn ask(&self) -> f64 {
        self.volume - self.bid
    }
}
