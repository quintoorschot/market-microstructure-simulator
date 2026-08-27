# 📈 Market Microstructure Simulator
**A Rust-based limit order book and matching engine for simulating exchange-style market microstructure.**

The project currently provides a limit order book, price-time-priority matching engine, event-driven simulation loop, and a simple trading agents.

## 🔎 Overview
Market microstructure focuses on how orders interact, trades are formed, and prices emerge within financial markets.

This project provides a foundation for exploring core market microstructure concepts, including:
- **Limit order book (LOB):** separate bid/ask books with FIFO price-time priority.
- **Matching engine:** supports partial fills and executions across multiple orders and price levels.
- **Order management:** submit, cancel, and modify orders with automatic queue-priority handling.
- **Discrete-event simulation:** process events deterministically by simulating time discretely.
- **Exchange events:** report events such as trades, cancellations, modifications.
- **Agent framework:** agents act autonomously, they submit orders schedule future actions based on current market conditions.

## ⚙️ Installation

### Clone the repository
```bash
git clone https://github.com/quintoorschot/market-microstructure-simulator.git
cd market-microstructure-simulator
```
### Build the project
```bash
cargo build
```
### Run the test suite
```bash
cargo test
```
### Run the example
```bash
cargo run
```
