mod candle;
mod position;

pub use candle::*;
pub use position::*;

use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Backtest {
    data: Vec<Candle>,
    position: Option<Position>,
    balance: f64,
    index: usize,
    position_history: Vec<PositionEvent>,
}

impl Backtest {
    pub fn new(data: Vec<Candle>, initial_balance: f64) -> Self {
        Self {
            data,
            position: None,
            balance: initial_balance,
            index: 0,
            position_history: Vec::new(),
        }
    }

    pub fn open_position(&mut self, position: Position) -> Result<()> {
        if self.position.is_some() {
            return Err(Error::msg("Already opened"));
        }

        let (side, price, quantity) =
            (position.side(), position.entry_price(), position.quantity());
        let cost = price * quantity;
        if self.balance < cost {
            return Err(Error::msg("Unbalanced wallet"));
        }

        match side {
            PositionSide::Long => self.balance -= cost,
            PositionSide::Short => self.balance += cost,
        }

        self.position = Some(Position::from((side.clone(), price, quantity)));
        self.position_history.push(PositionEvent::from((
            self.index.saturating_sub(1),
            price,
            PositionEventType::Open(side),
        )));

        Ok(())
    }

    pub fn close_position(&mut self, exit_price: f64) -> Result<f64> {
        if let Some(position) = self.position.take() {
            let value = match position.side() {
                PositionSide::Long => {
                    let value = exit_price * position.quantity();
                    self.balance += value;
                    value
                }
                PositionSide::Short => {
                    self.balance -= position.entry_price() * position.quantity();
                    let profit = (position.entry_price() - exit_price) * position.quantity();
                    self.balance += profit;
                    profit
                }
            };

            self.position_history.push(PositionEvent::from((
                self.index.saturating_sub(1),
                exit_price,
                PositionEventType::Close,
            )));
            return Ok(value);
        }

        Err(Error::msg("No opened position"))
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Iterator for Backtest {
    type Item = Candle;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.data.get(self.index).cloned();
        self.index += 1;
        item
    }
}

#[cfg(test)]
mod tests {
    use super::PositionSide::*;
    use super::*;

    fn get_data() -> Vec<Candle> {
        vec![
            Candle::from((100.0, 111.0, 99.0, 110.0, 1.0)),
            Candle::from((110.0, 112.0, 100.0, 120.0, 1.0)),
            Candle::from((120.0, 121.0, 100.0, 110.0, 1.0)),
        ]
    }

    #[test]
    fn test_long_position() {
        let data = get_data();
        let balance = 1000.0;
        let mut bt = Backtest::new(data, balance);
        let mut _counter = 0;
        while let Some(candle) = bt.next() {
            let price = candle.close();
            if _counter == 0 {
                let result = bt.open_position((Long, price, 1.0).into()); // balance (1000.0) -= 110.0 * 1.0 => 890.0;
                assert!(result.is_ok());
                assert!(bt.position.is_some());
                assert!(bt.balance < balance);
            }
            if _counter == 1 {
                let result = bt.close_position(price);
                assert!(result.is_ok());
                assert!(bt.position.is_none());
                assert!(bt.balance > balance);
            }
            _counter += 1;
        }
    }

    #[test]
    fn test_short_position() {
        let data = get_data();
        let balance = 1000.0;
        let mut bt = Backtest::new(data, balance);
        let mut _counter = 0;
        while let Some(candle) = bt.next() {
            let price = candle.close();
            if _counter == 1 {
                let result = bt.open_position((Short, price, 1.0).into());
                assert!(result.is_ok());
                assert!(bt.position.is_some());
                assert!(bt.balance > balance);
            }
            if _counter == 2 {
                let result = bt.close_position(price);
                assert!(result.is_ok());
                assert!(bt.position.is_none());
                assert!(bt.balance > balance);
            }
            _counter += 1;
        }
    }

    #[test]
    fn test_failed_long_position() {
        let data = get_data();
        let balance = 1000.0;
        let mut bt = Backtest::new(data, balance);
        let mut _counter = 0;
        while let Some(candle) = bt.next() {
            let price = candle.close();
            if _counter == 1 {
                let result = bt.open_position((Long, price, 1.0).into()); // balance (1000.0) -= 110.0 * 1.0 => 890.0;
                assert!(result.is_ok());
                assert!(bt.position.is_some());
                assert!(bt.balance < balance);
            }
            if _counter == 2 {
                let result = bt.close_position(price);
                assert!(result.is_ok());
                assert!(bt.position.is_none());
                assert!(bt.balance < balance);
            }
            _counter += 1;
        }
    }

    #[test]
    fn test_failed_short_position() {
        let data = get_data();
        let balance = 1000.0;
        let mut bt = Backtest::new(data, balance);
        let mut _counter = 0;
        while let Some(candle) = bt.next() {
            let price = candle.close();
            if _counter == 0 {
                let result = bt.open_position((Short, price, 1.0).into());
                assert!(result.is_ok());
                assert!(bt.position.is_some());
                assert!(bt.balance > balance);
            }
            if _counter == 1 {
                let result = bt.close_position(price);
                assert!(result.is_ok());
                assert!(bt.position.is_none());
                assert!(bt.balance < balance);
            }
            _counter += 1;
        }
    }
}
