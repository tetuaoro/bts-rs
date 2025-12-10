# BTS TODO

## ✅ Done
- [x] Wallet management
- [x] Position management
- [x] Order management system
- [x] Market simulation engine
- [x] Market fees implementation
- [x] Add `Candle` builder for validation
- [x] Add `Metrics` struct to wrap metrics (P&L, drawdown, Sharpe)
- [x] Timeframe/Volume aggregation (1H → 4H/8H/1D or 1D → 7D/1M)
- [x] Parameters optimization
- [x] Strategy examples (5+ templates)
- [x] WASM compilation support
- [x] Automated report generation (PDF/HTML)
- [x] Optimize (use Arc<[Candle]>, impl Copy trait) and remove unwrap and clone (partial)

## 📌 In Progress
- [ ] Write better docs, examples and tests
- [ ] Tracing and Progress when running strategy

## 🚀 Road to v1.0.0
- [ ] Add methods to modify orders/positions (update SL/TP/trailing stop)
- [ ] Multi-strategy parallel execution

### Core Features

### Advanced Features
- [ ] Web/Desktop App dashboard integration