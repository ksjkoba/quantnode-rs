# QuantNode-RS

A low-latency quantitative finance execution node built in Rust. This project focuses on high-speed data ingestion and safe concurrent execution.

## Why Rust?
* **Zero-cost abstractions:** Maximum performance without the overhead of a garbage collector.
* **Fearless Concurrency:** Rust's ownership model ensures no data races when processing multiple market feeds.

## Project Structure
* src/main.rs: Entry point and execution loop.
* Cargo.toml: Dependency and build configuration.
