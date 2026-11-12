
# Rust Port Scanner

A fast, multi-threaded port scanner built with Rust. This tool allows you to quickly check for open TCP ports on a target host.

## Features

- **Multi-threaded:** Scans multiple ports simultaneously using `std::thread`.
- **Timeout support:** Prevents the scanner from hanging on closed ports.
- **Concurrency:** Uses `Arc` and `Mutex` to safely collect results from multiple threads.
- **File Output:** Saves all open ports to a text file (`scan_results.txt`) for later analysis.

## Prerequisites

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

