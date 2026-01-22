# Project Euler in Rust (Modern)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

This repository contains Rust implementations of selected Project Euler problems, rewritten and organized using modern Rust practices.

The primary goal of this project is not just to solve Project Euler problems, but to use them as a structured and progressive way to (re)learn Rust in a clean, idiomatic, and maintainable manner.

This is ported from the older, archived repo - https://github.com/pythonhacker/project-euler-rust

## Project Structure

The repository is organized as a Cargo workspace with two crates:

```
project-euler-rust/
├── Cargo.toml          # Workspace definition
├── euler-lib/          # Shared utilities and helpers
│   └── src/lib.rs
└── euler-solutions/    # Individual problem solutions (binaries)
    └── src/bin/
        ├── problem1.rs
        ├── problem2.rs
        └── ...
```

### Crates

- euler-lib  
  Common utilities such as math helpers, prime generation, and BigInt-based routines shared across problems.

- euler-solutions  
  Each Project Euler problem is implemented as a separate binary under src/bin/.

## Building the Project

From the root of the repository:

```
cargo build
```

## Running a Specific Problem

Each problem is a standalone binary. For example:

```
cargo run --bin problem1
```

Or if you like being more verbose,

```
cargo run -p euler-solutions --bin problem1
```

Replace `problem1` with the desired problem number.

## Goals of This Project

- Use Project Euler as a practical Rust learning framework
- Explore idiomatic Rust design progressively
- Keep solutions clean, testable, and maintainable
- Separate reusable logic from problem-specific code
- Gradually refactor and improve older Rust code using modern patterns

## Roadmap

This project is being evolved in phases:

1. Port existing solutions into a modern Rust workspace layout
2. Analyze and categorize problems by Rust learning value
3. Selectively rewrite problems to improve idiomatic usage
4. Introduce benchmarks, tests, and optimizations over time

## License

Licensed under the Mozilla Public License, Version 2.0 (MPL-2.0).
