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
## 🙏 Acknowledgements

This project was developed alongside my ongoing studies at Eindhoven University of Technology (TU/e), combining academic learning with independent research and implementation. The project was informed in part by material from MIT OpenCourseWare, alongside my own exploration of financial market dynamics, electronic order books, and market microstructures.

<p align="center">
  <a href="https://ocw.mit.edu/">
    <img
      src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQbXwMQlvlr3o5wbotF_9b9uXMBJF-WIJ3sV9BNHsOaBz-ZO2A53LBdQMxP&s=10"
      alt="MIT OpenCourseWare logo"
      width="200"
    />
  </a>
  <span>&nbsp;&nbsp;&nbsp;&nbsp;</span>
  <a href="https://www.tue.nl/en/">
    <img
      src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS0as6Zrck2G2too3qxUP5-3iTE7Mb7_CljRxOR_R8aIkMtAUozwpA0jKZ5&s=10"
      alt="TU Eindhoven logo"
      width="200"
    />
  </a>
</p>
