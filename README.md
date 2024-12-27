# RustGenericBlockchain Project

## Overview
This project implements a flexible and generic blockchain system in Rust using `Generics` and `Config Traits`. It focuses on creating reusable and configurable pallets for managing user balances and blockchain states (block numbers and nonces). The project demonstrates best practices in Rust for modularity, maintainability, and type safety.

---

## Features

### 1. **System Pallet**
- Manages:
  - **Block Numbers**: Tracks the progression of blockchain states.
  - **Nonces**: Ensures unique transaction identifiers for each user.
- Implements generics to allow configurable types for `AccountId`, `BlockNumber`, and `Nonce`.
- Supports incrementing block numbers and nonces.

### 2. **Balances Pallet**
- Manages:
  - **User Balances**: Tracks balances for each account.
  - **Transfers**: Securely transfers funds between accounts with error handling.
- Implements generics for `AccountId` and `Balance` types, allowing type flexibility.
- Ensures type safety with constraints like `AddAssign`, `SubAssign`, and `PartialOrd`.

### 3. **Runtime**
- Integrates:
  - **System Pallet**: For block and nonce management.
  - **Balances Pallet**: For user account management.
- Centralizes configurations using `Config Traits` for type definitions.
- Provides a single interface to simulate blockchain transactions.

### 4. **Generics and Config Traits**
- Generic implementations for pallets ensure type flexibility.
- `Config Traits` encapsulate type definitions, reducing redundancy and enhancing modularity.
- Enables:
  - **Custom Types**: Define types like `AccountId`, `BlockNumber`, and `Balance` at runtime.
  - **Type Inheritance**: Reuses `Config Traits` across pallets to reduce code duplication.

### 5. **Debugging**
- Includes a macro `print_runtime_state!` for easy debugging of the runtime state.
- Outputs the current state of the blockchain, including block numbers, nonces, and balances.

---

## How to Run

### Prerequisites
- Install Rust: [Rust Installation](https://www.rust-lang.org/tools/install)

### Steps
1. **Clone the Repository**:
   ```bash
   git clone <repository_url>
   cd RustGenericBlockchain
   ```

2. **Add External Dependency**:
   - This project uses the `num` library for numerical operations. Add it using:
     ```bash
     cargo add num
     ```

3. **Build and Run**:
   ```bash
   cargo build
   cargo run
   ```

4. **Run Tests**:
   ```bash
   cargo test
   ```

---

## Example Output

### Running the Program
The program simulates blockchain transactions with real-time state updates.

```yaml
Transaction successful! Alice sent 300 to Bob.
Updated Runtime State:
Runtime {
    system: System {
        block_number: 1,
        transaction_count: {"Alice": 1},
    },
    balances: Balances {
        balances: {"Alice": 700, "Bob": 800},
    },
}
Transaction successful! Bob sent 200 to Alice.
Updated Runtime State:
Runtime {
    system: System {
        block_number: 2,
        transaction_count: {"Alice": 1, "Bob": 1},
    },
    balances: Balances {
        balances: {"Alice": 900, "Bob": 600},
    },
}
```

---

## Highlights

### **Modularity**
- Separate pallets for `System` and `Balances` ensure code clarity and reusability.

### **Type Safety**
- Uses Rust's generic system and traits to enforce type constraints and prevent runtime errors.

### **Flexibility**
- Configurable types (`AccountId`, `BlockNumber`, `Nonce`, `Balance`) allow seamless integration with various runtime environments.

---

## License
This project is licensed under the MIT License.
```

---

### **Key Features in the README**
1. **Overview**:
   - Summarizes the project's scope and goals.
2. **Features**:
   - Breaks down key components and their roles.
3. **How to Run**:
   - Step-by-step instructions for building, running, and testing.
4. **Example Output**:
   - Demonstrates the expected behavior of the program.
5. **Highlights**:
   - Emphasizes the modularity, type safety, and flexibility of the implementation.