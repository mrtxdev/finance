# Fiscal - Personal Finance Manager

A modern, high-performance desktop personal finance manager built in Rust. The application features a lightweight graphical user interface (GUI) powered by `egui` and a decoupled modular architecture to handle strict core business logic, automated local persistence via dynamic JSON serialization, and precise calculations.

---

## Table of Contents

- [Key Features](#key-features)
- [Architecture Overview](#architecture-overview)
- [Data Visualizations and UI](#data-visualizations-and-ui)
- [Data Models and Formats](#data-models-and-formats)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Running Automated Tests](#running-automated-tests)
- [License](#license)

---

## Key Features

- **Decoupled Architecture:** Pure domain logic isolated from presentation frameworks.
- **Dynamic Balance Calculation:** Real-time state mutability reflecting global balances instantly.
- **Subscription Filtering:** Native encapsulation to parse and isolate active recurring transactions.
- **Robust Persistence Model:** Custom serialization adapters with automatic I/O error bubbling.
- **Sleek Desktop Layout:** Native hardware-accelerated dark theme configured for high scannability.

---

## Architecture Overview

The system is configured as a Cargo Workspace divided into two distinct decoupled layers:

```
finance/
├── core/               # Domain logic layer (Library crate)
│   ├── src/
│   │   ├── finance.rs  # State management and file I/O operations
│   │   ├── model.rs    # Data models and structures
│   │   └── serializers.rs # Custom serialization/deserialization adapters
│   └── tests/          # Automated backend verification test suites
├── gui/                # Desktop user interface layer (Binary crate)
│   └── src/
│       ├── app.rs      # Global application state management
│       ├── main.rs     # Graphical framework initialization entrypoint
│       └── ui/         # Modular GUI component templates
└── Cargo.toml          # Workspace orchestrator
```

- **`core` (Domain Module):** Handles algebraic balance calculations, data vector filtering (`retain`, `filter`), internal ID generation using Unix Epoch offsets, and structural conversions (`from_str`, `to_string`).
- **`gui` (Presentation Module):** Manages interactive user frames, immediate-mode widget pipelines, conditional validation vectors, and component event delegation loops.

---

## Data Visualizations and UI

The user interface uses visual separation blocks to guide user focus through critical financial metadata layouts.

### Status Indicators and Summaries

| Total Balance | Subscriptions Total | Active Subscriptions |
| :--- | :--- | :--- |
| `$3,439.50` | `$180.00` | `3` |

### Spending Distribution Analytics

The reporting views use horizontal analytical progress structures to denote exact expense density categories across historical logs:

* **Food:** `[████░░░░░░░░░░░░░░░░] 15%`
* **Transport:** `[█████████░░░░░░░░░░░] 45%`
* **Leisure:** `[█░░░░░░░░░░░░░░░░░░░] 5%`
* **Subscription:** `[██████░░░░░░░░░░░░░░] 30%`

---

## Data Models and Formats

Local persistence records transaction structural indices into a continuous file stream named `data.json`. Financial payloads adhere to the following schema definition:

```json
{
  "transactions": [
    {
      "id": 1726574312000,
      "amount": 2500.0,
      "description": "Freelance project payment",
      "category": "Salary",
      "is_subscription": false,
      "date": "17/09/2026"
    }
  ]
}
```

### Supported Categories

* `Salary` (Positive inflow values)
* `Food` (Negative expense values)
* `Transport` (Negative expense values)
* `Leisure` (Negative expense values)
* `Subscription` (Negative automated expense values)
* `Other` (General fallback items)

---

## Getting Started

### Prerequisites

Ensure you have the latest stable Rust toolchain installed on your host machine.

```bash
rustc --version
cargo --version
```

### Installation

1. Clone the repository to your environment:
   ```bash
   git clone https://github.com/mrtxdev/finance.git
   cd finance
   ```

2. Compile and run the desktop application target using the Cargo workspace wrapper:
   ```bash
   cargo run --package gui
   ```

---

## Running Automated Tests

The core architecture maintains automated unit and integration tests verifying logical calculation continuity, file handling stability, deletion behavior routines, and explicit serialization outputs.

Execute the following test orchestration command to check the integrity constraints:

```bash
cargo test
```

### Test Coverage Targets

* Balance sum verification checking accuracy down to double-precision float representations.
* Functional filtering validation checking transaction reference mapping arrays.
* Memory mutation vectors confirming safe item eviction sequences using `retain` logic.
* Automated error trait conversions using uniform opaque object wrappers.

---

## License

This software project is licensed under the standard repository distribution terms. Feel free to clone, adapt, and expand.
