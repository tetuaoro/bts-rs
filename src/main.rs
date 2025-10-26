mod engine;
mod plot;
mod utils;

use crate::engine::*;
use crate::utils::*;

use anyhow::*;

fn main() -> Result<()> {
    let items = get_data_from_file("data/btc.json".into())?;

    let candles = items
        .iter()
        .map(|d| Candle::from((d.open(), d.high(), d.low(), d.close(), d.volume())))
        .collect::<Vec<_>>();

    let bt = Backtest::new(candles, 1000.0);
    bt.for_each(|d| println!("{d:?}"));

    Ok(())
}
