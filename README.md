# Blockchain Middleware – Binary Stream Ingestion Engine

## Overview

This project demonstrates the design of an off-chain middleware component for ingesting and processing a custom binary protocol from a high-throughput blockchain node.

The system consists of two components:

- **Python Generator** — Simulates the raw TCP binary stream by generating `stream.bin`
- **Rust Ingestion Engine** — Parses, validates, filters, and outputs structured JSON data

Only transaction events with `amount > 1000` are emitted.

---

## Architecture
blockchain-middleware/
├── generator/ # Python binary stream generator
│ └── generator.py
│
├── ingestion-engine/ # Rust ingestion engine
│ ├── Cargo.toml
│ └── src/
│ └── main.rs
│
└── README.md


---

## Protocol Specification (Legacy Format)

Each packet follows this binary structure (Big Endian):

| Field          | Size        | Description |
|---------------|------------|-------------|
| Magic Byte     | 1 byte     | Always `0xA5` |
| Packet Type    | 1 byte     | `0x01`, `0x02`, `0xFF` |
| Payload Length | 2 bytes    | Unsigned integer (Big Endian) |
| Payload        | Variable   | Packet-specific data |
| Checksum       | 1 byte     | XOR of all payload bytes |

---

## Packet Types

### `0x01` – Transaction Event
- 32 bytes: TxHash  
- 8 bytes: Amount (u64)  
- Remaining bytes: UTF-8 Memo string  

### `0x02` – State Update
- 32 bytes: BlockHash  
- 1 byte: Status  

### `0xFF` – Keep-Alive
- Deprecated legacy payload  

---

## Validation Logic

The Rust ingestion engine performs:

- Magic byte validation
- Payload length verification
- XOR checksum validation
- Safe handling of truncated packets
- Graceful skipping of unsupported packet types

Invalid packets are ignored without crashing the application.

---

## Filtering Logic

Only transaction events where:

```rust
amount > 1000


---

## Protocol Specification (Legacy Format)

Each packet follows this binary structure (Big Endian):

| Field          | Size        | Description |
|---------------|------------|-------------|
| Magic Byte     | 1 byte     | Always `0xA5` |
| Packet Type    | 1 byte     | `0x01`, `0x02`, `0xFF` |
| Payload Length | 2 bytes    | Unsigned integer (Big Endian) |
| Payload        | Variable   | Packet-specific data |
| Checksum       | 1 byte     | XOR of all payload bytes |

---

## Packet Types

### `0x01` – Transaction Event
- 32 bytes: TxHash  
- 8 bytes: Amount (u64)  
- Remaining bytes: UTF-8 Memo string  

### `0x02` – State Update
- 32 bytes: BlockHash  
- 1 byte: Status  

### `0xFF` – Keep-Alive
- Deprecated legacy payload  

---

## Validation Logic

The Rust ingestion engine performs:

- Magic byte validation
- Payload length verification
- XOR checksum validation
- Safe handling of truncated packets
- Graceful skipping of unsupported packet types

Invalid packets are ignored without crashing the application.

---

## Filtering Logic

Only transaction events where:

```rust
amount > 1000




