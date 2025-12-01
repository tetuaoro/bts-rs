//! Strategy parameter optimization.
//!
//! This module provides tools to optimize trading strategies by testing different parameter combinations.
//! The `Optimizer` struct handles the execution of backtests for each combination, while the
//! `ParameterCombination` trait defines how to generate parameter sets.

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::engine::{Backtest, Candle};
use crate::errors::Result;

use rayon::prelude::*;

/// Trait defining how to generate parameter combinations for optimization.
///
/// Implement this trait for your parameter types to define how combinations should be generated.
/// The associated type `P` represents a single parameter combination (e.g., a tuple of values).
pub trait ParameterCombination {
    /// Type representing a single parameter combination (e.g., `(usize, f64)`).
    type T: Clone + Sync;

    /// Generates all possible parameter combinations to test.
    ///
    /// # Returns
    /// A vector containing all parameter combinations.
    fn generate() -> Vec<Self::T>;
}

/// Optimizer for testing trading strategies with different parameter combinations.
///
/// This struct handles the execution of backtests for each parameter combination,
/// collecting results for analysis.
pub struct Optimizer<PS: ParameterCombination> {
    data: Vec<Candle>,
    initial_balance: f64,
    _marker: PhantomData<PS>,
    market_fees: Option<(f64, f64)>,
}

impl<PS: ParameterCombination> Optimizer<PS> {
    /// Creates a new `Optimizer` with the given data and initial balance.
    ///
    /// # Arguments
    /// * `data` - Historical candle data for backtesting.
    /// * `initial_balance` - Starting balance for the backtest.
    /// * `market_fees` - Optional tuple of (maker fee, taker fee).
    ///
    /// # Returns
    /// A new `Optimizer` instance.
    pub fn new(data: Vec<Candle>, initial_balance: f64, market_fees: Option<(f64, f64)>) -> Self {
        Self {
            data,
            market_fees,
            initial_balance,
            _marker: PhantomData,
        }
    }

    /// Optimizes a trading strategy by testing all parameter combinations.
    ///
    /// # Type Parameters
    /// * `S` - Strategy function type.
    /// * `I` - Parameter setter function type.
    /// * `T` - Type of the configured parameters passed to the strategy.
    ///
    /// # Arguments
    /// * `setters` - Function that converts a parameter combination into strategy-specific parameters.
    /// * `strategy` - Trading strategy function to test.
    ///
    /// # Returns
    /// A vector of tuples containing each parameter combination and its resulting balance.
    ///
    /// # Errors
    /// Returns an error if backtest execution fails.
    pub fn with<S, TR, T>(&self, transformers: TR, strategy: S) -> Result<Vec<(PS::T, f64)>>
    where
        T: Clone,
        PS: Sync,
        PS::T: Send,
        TR: Fn(&PS::T) -> Result<T> + Sync,
        S: FnMut(&mut Backtest, &mut T, &Candle) -> Result<()> + Send,
    {
        let num_cpus = num_cpus::get();
        let combinations = PS::generate();

        let error = Mutex::new(None);
        let results = Mutex::new(Vec::with_capacity(combinations.len()));

        let strategy = Arc::new(Mutex::new(strategy));
        let chunk_size = ((combinations.len() + num_cpus - 1) / num_cpus).max(1);

        //todo optimize and remove unwrap

        combinations.par_chunks(chunk_size).for_each(|par_combinations| {
            if error.lock().unwrap().is_some() {
                return;
            }

            let mut local_results = Vec::with_capacity(par_combinations.len());
            let mut backtest = match Backtest::new(self.data.clone(), self.initial_balance, self.market_fees) {
                Ok(bt) => bt,
                Err(e) => {
                    *error.lock().unwrap() = Some(e);
                    return;
                }
            };

            for param_set in par_combinations {
                let mut transformer = match transformers(param_set) {
                    Ok(s) => s,
                    Err(e) => {
                        *error.lock().unwrap() = Some(e);
                        return;
                    }
                };

                let strategy_arc = Arc::clone(&strategy);
                let mut strategy_guard = strategy_arc.lock().unwrap();

                if let Err(e) = backtest.run(|bt, candle| strategy_guard(bt, &mut transformer, candle)) {
                    *error.lock().unwrap() = Some(e);
                    return;
                }

                let current_balance = backtest.total_balance();

                local_results.push((param_set.clone(), current_balance));

                backtest.reset();
            }

            results.lock().unwrap().extend(local_results);
        });

        if let Some(e) = error.into_inner().unwrap() {
            return Err(e);
        }

        Ok(results.into_inner().unwrap())
    }
}
