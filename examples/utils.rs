use bts_rs::engine::{Candle, CandleBuilder};
#[cfg(feature = "metrics")]
use bts_rs::metrics::Metrics;
use chrono::{DateTime, Duration};

/// Generates deterministic candle data.
pub fn generate_sample_candles(max: i32, seed: i32, base_price: f64) -> Vec<Candle> {
    let mut open_time = DateTime::default();
    let mut open = base_price;

    (0..=max)
        .map(|i| {
            // Base price with trend (+ 0.5*i)
            let base_price = base_price + 0.5 * (i as f64);

            // Price variation using simple trigonometric function with seed
            let variation = 5.0 * ((i as f64 * 0.3 + seed as f64).sin() * 0.5 + 0.5);

            // Calculate OHLC prices
            let close = base_price + variation;
            let high = close + 0.3 * variation.abs();
            let low = close - 0.3 * variation.abs();
            // Ensure valid price order: open ≤ low ≤ high ≤ close
            let low = low.min(open);
            let high = high.max(close);
            // Volume with seasonal pattern
            let volume = 1000.0 + 500.0 * ((i as f64 * 0.2).sin()).abs();
            // Bid price (slightly below close)
            let bid = close * 0.999;

            let close_time = open_time + Duration::days(1);

            let candle = CandleBuilder::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .bid(bid)
                .open_time(open_time)
                .close_time(close_time)
                .build()
                .unwrap();

            open_time = close_time + Duration::microseconds(1);
            open = close;
            candle
        })
        .collect()
}

pub fn example_candles() -> Vec<Candle> {
    generate_sample_candles(3000, 42, 100.0)
}

/// Pretty print Metrics
#[cfg(feature = "metrics")]
#[allow(dead_code)]
pub fn print_metrics(metrics: &Metrics, initial_balance: f64) {
    println!("=== Backtest Metrics ===");
    println!("Initial Balance: {:.2}", initial_balance);
    println!("Max Drawdown: {:.2}%", metrics.max_drawdown());
    println!("Profit Factor: {:.2}", metrics.profit_factor());
    println!("Sharpe Ratio (risk-free rate = 2%): {:.2}", metrics.sharpe_ratio(0.02));
    println!("Win Rate: {:.2}%", metrics.win_rate());
}

#[macro_export]
/// Pause and resume when press any key.
macro_rules! pause {
    () => {
        println!("[{}:{}] Pausing! Press enter to continue...", file!(), line!());
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
    };
}

#[allow(dead_code)]
fn main() {}
