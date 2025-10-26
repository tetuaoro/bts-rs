#[derive(Debug, Clone, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Debug, Clone)]
pub struct Position {
    side: PositionSide,
    entry_price: f64,
    quantity: f64,
}

impl Position {
    pub fn side(&self) -> PositionSide {
        self.side.clone()
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn entry_price(&self) -> f64 {
        self.entry_price
    }
}

impl From<(PositionSide, f64, f64)> for Position {
    fn from((side, entry_price, quantity): (PositionSide, f64, f64)) -> Self {
        Self {
            side,
            entry_price,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PositionEventType {
    Open(PositionSide),
    Close,
}

#[derive(Debug, Clone)]
pub struct PositionEvent {
    candle_index: usize,
    price: f64,
    event_type: PositionEventType,
}

impl From<(usize, f64, PositionEventType)> for PositionEvent {
    fn from((index, price, event): (usize, f64, PositionEventType)) -> Self {
        Self {
            candle_index: index,
            price,
            event_type: event,
        }
    }
}
