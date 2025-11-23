use std::fmt;

use crate::engine::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    AddOrder(Order),
    DelOrder(Order),
    AddPosition(Position),
    DelPosition(Position),
    WalletUpdate {
        pnl: f64,
        fees: f64,
        free: f64,
        locked: f64,
        balance: f64,
    },
    FeeCharged {
        fee: f64,
        order_id: u32,
    },
}

/// A collection of trading metrics calculated from a series of events.
///
/// `Metrics` is used to compute and display key performance indicators (KPIs)
/// for a trading strategy, such as max drawdown, profit factor, Sharpe ratio, and win rate.
/// It is typically constructed from a `Backtest` or a list of `Event`s.
#[derive(Debug)]
pub struct Metrics {
    events: Vec<Event>,
    initial_balance: f64,
}

impl From<&Backtest> for Metrics {
    fn from(value: &Backtest) -> Self {
        Self {
            events: value.events().cloned().collect::<Vec<_>>(),
            initial_balance: value.initial_balance(),
        }
    }
}

impl Metrics {
    /// Creates a new `Metrics` instance from a list of events and an initial balance.
    ///
    /// ### Arguments
    ///
    /// * `events` - A vector of `Event`s to analyze.
    /// * `initial_balance` - The initial balance of the trading account.
    ///
    /// ### Returns
    ///
    /// A new `Metrics` instance.
    pub fn new(events: Vec<Event>, initial_balance: f64) -> Self {
        Self {
            events,
            initial_balance,
        }
    }

    /// Computes the maximum drawdown as a percentage.
    ///
    /// Drawdown is the peak-to-trough decline in the account balance.
    pub fn max_drawdown(&self) -> f64 {
        let mut balance_history = Vec::new();
        #[allow(unused_assignments)]
        let mut current_balance = self.initial_balance;

        for event in &self.events {
            if let Event::WalletUpdate { balance, .. } = event {
                current_balance = *balance;
                balance_history.push(current_balance);
            }
        }

        let mut max_peak = self.initial_balance;
        let mut max_drawdown = 0.0;

        for &balance in &balance_history {
            if balance > max_peak {
                max_peak = balance;
            }
            let drawdown = (max_peak - balance) / max_peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }

        max_drawdown * 100.0
    }

    /// Computes the profit factor.
    ///
    /// Profit factor is the ratio of gross profits to gross losses.
    /// A profit factor greater than 1.0 indicates a profitable strategy.
    pub fn profit_factor(&self) -> f64 {
        let mut total_gains = 0.0;
        let mut total_losses = 0.0;

        for event in &self.events {
            if let Event::DelPosition(position) = event {
                let pnl = position.pnl().unwrap_or(0.0);
                if pnl > 0.0 {
                    total_gains += pnl;
                } else {
                    total_losses += pnl.abs();
                }
            }
        }

        if total_losses == 0.0 {
            return f64::INFINITY;
        }

        total_gains / total_losses
    }

    /// Computes the Sharpe ratio, a measure of risk-adjusted return.
    ///
    /// A higher Sharpe ratio indicates better risk-adjusted performance.
    /// `risk_free_rate` is the annualized risk-free return (e.g., 0.0 for simplicity).
    pub fn sharpe_ratio(&self, risk_free_rate: f64) -> f64 {
        let mut returns = Vec::new();
        let mut previous_balance = self.initial_balance;

        for event in &self.events {
            if let Event::WalletUpdate { balance, .. } = event {
                let return_pct = (*balance - previous_balance) / previous_balance;
                returns.push(return_pct);
                previous_balance = *balance;
            }
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let std_dev = (returns.iter().map(|r| (r - mean_return).powi(2)).sum::<f64>() / returns.len() as f64).sqrt();

        (mean_return - risk_free_rate) / std_dev
    }

    /// Computes the win rate as a percentage of winning trades.
    pub fn win_rate(&self) -> f64 {
        let mut winning_trades = 0;
        let mut total_trades = 0;

        for event in &self.events {
            if let Event::DelPosition(position) = event {
                total_trades += 1;
                if position.pnl().unwrap_or(0.0) > 0.0 {
                    winning_trades += 1;
                }
            }
        }

        if total_trades == 0 {
            return 0.0;
        }

        (winning_trades as f64 / total_trades as f64) * 100.0
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Backtest Metrics ===")?;
        writeln!(f, "Initial Balance: {:.2}", self.initial_balance)?;
        writeln!(f, "Max Drawdown: {:.2}%", self.max_drawdown())?;
        writeln!(f, "Profit Factor: {:.2}", self.profit_factor())?;
        writeln!(f, "Sharpe Ratio (risk-free rate = 0.0): {:.2}", self.sharpe_ratio(0.0))?;
        writeln!(f, "Win Rate: {:.2}%", self.win_rate())?;
        Ok(())
    }
}

#[cfg(test)]
// Helper function to create a simple position for testing
fn create_position(pnl: f64) -> Position {
    let order: Order = (OrderType::Market(100.0), 1.0, OrderSide::Buy).into();
    let mut position = Position::from(order);
    // Mock the pnl() method for testing
    // In a real scenario, you would set the exit price or mock the behavior
    // Here, we assume Position has a method to set pnl directly for testing
    position.set_exit_price(100.0 + pnl); // Simulate a P&L of `pnl`
    position
}

#[cfg(test)]
#[test]
fn max_drawdown() {
    let events = vec![
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 10000.0,
            locked: 0.0,
            balance: 10000.0,
        },
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 12000.0,
            locked: 0.0,
            balance: 12000.0,
        },
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 9000.0,
            locked: 0.0,
            balance: 9000.0,
        },
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 11000.0,
            locked: 0.0,
            balance: 11000.0,
        },
    ];
    let metrics = Metrics::new(events, 10000.0);
    assert_eq!(metrics.max_drawdown(), 25.0); // (12000 - 9000) / 12000 = 25%
}

#[cfg(test)]
#[test]
fn max_drawdown_no_events() {
    let metrics = Metrics::new(vec![], 10000.0);
    assert_eq!(metrics.max_drawdown(), 0.0); // No drawdown if no events
}

#[cfg(test)]
#[test]
fn profit_factor() {
    let winning_position = create_position(20.0);
    let losing_position = create_position(-10.0);
    let events = vec![
        Event::DelPosition(winning_position),
        Event::DelPosition(losing_position),
    ];
    let metrics = Metrics::new(events, 10000.0);
    assert_eq!(metrics.profit_factor(), 2.0); // 20 / 10 = 2.0
}

#[cfg(test)]
#[test]
fn profit_factor_no_losses() {
    let winning_position = create_position(20.0);
    let events = vec![Event::DelPosition(winning_position)];
    let metrics = Metrics::new(events, 10000.0);
    assert_eq!(metrics.profit_factor(), f64::INFINITY); // No losses
}

#[cfg(test)]
#[test]
fn profit_factor_no_trades() {
    let metrics = Metrics::new(vec![], 10000.0);
    assert_eq!(metrics.profit_factor(), f64::INFINITY); // No trades
}

#[cfg(test)]
#[test]
fn sharpe_ratio() {
    let events = vec![
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 10000.0,
            locked: 0.0,
            balance: 10000.0,
        },
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 10500.0,
            locked: 0.0,
            balance: 10500.0,
        },
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 10300.0,
            locked: 0.0,
            balance: 10300.0,
        },
        Event::WalletUpdate {
            pnl: 0.0,
            fees: 0.0,
            free: 10700.0,
            locked: 0.0,
            balance: 10700.0,
        },
    ];
    let metrics = Metrics::new(events, 10000.0);
    let sharpe = metrics.sharpe_ratio(0.0);
    // Approximate value, since Sharpe ratio depends on standard deviation
    assert!(sharpe > 0.0 && sharpe < 1.0);
}

#[cfg(test)]
#[test]
fn sharpe_ratio_no_events() {
    let metrics = Metrics::new(vec![], 10000.0);
    // Sharpe ratio is undefined (division by zero), but in practice, it will return NaN
    assert!(metrics.sharpe_ratio(0.0).is_nan());
}

#[cfg(test)]
#[test]
fn win_rate() {
    let winning_position = create_position(20.0);
    let losing_position = create_position(-10.0);
    let events = vec![
        Event::DelPosition(winning_position),
        Event::DelPosition(losing_position),
    ];
    let metrics = Metrics::new(events, 10000.0);
    assert_eq!(metrics.win_rate(), 50.0); // 1 win out of 2 trades
}

#[cfg(test)]
#[test]
fn win_rate_no_trades() {
    let metrics = Metrics::new(vec![], 10000.0);
    assert_eq!(metrics.win_rate(), 0.0); // No trades
}

#[cfg(test)]
#[test]
fn win_rate_all_winning() {
    let winning_position = create_position(20.0);
    let events = vec![Event::DelPosition(winning_position)];
    let metrics = Metrics::new(events, 10000.0);
    assert_eq!(metrics.win_rate(), 100.0); // 1 win out of 1 trade
}
