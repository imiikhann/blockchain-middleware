Blockchain Middleware – Binary Stream Ingestion Engine
Overview

This project demonstrates the design of an off-chain middleware component for ingesting and processing a custom binary protocol from a high-throughput blockchain node.

The system consists of two components:

Python Generator — Simulates the raw TCP binary stream by generating stream.bin

Rust Ingestion Engine — Parses, validates, filters, and outputs structured JSON data

Only transaction events with amount > 1000 are emitted.

Architecture
blockchain-middleware/
├── generator/           # Python binary stream generator
│   └── generator.py
│
├── ingestion-engine/    # Rust ingestion engine
│   ├── Cargo.toml
│   └── src/main.rs
│
└── README.md
Protocol Specification (Legacy Format)

Each packet follows this binary structure (Big Endian):

Field	Size	Description
Magic Byte	1 byte	Always 0xA5
Packet Type	1 byte	0x01, 0x02, 0xFF
Payload Length	2 bytes	Unsigned integer
Payload	Variable	Packet-specific data
Checksum	1 byte	XOR of all payload bytes
Packet Types
0x01 – Transaction Event

32 bytes: TxHash

8 bytes: Amount (u64)

Remaining bytes: UTF-8 Memo string

0x02 – State Update

32 bytes: BlockHash

1 byte: Status

0xFF – Keep-Alive

Deprecated legacy payload

Validation Logic

The Rust ingestion engine performs:

Magic byte validation

Payload length validation

Checksum verification (XOR)

Safe handling of truncated packets

Graceful skipping of unsupported packet types

Invalid packets are ignored without crashing the application.

Filtering Logic

Only transaction events where:

amount > 1000

are emitted as JSON to stdout.

Prerequisites

Python 3.9+

Rust (stable toolchain)

Cargo

How To Run
1. Generate Binary Stream
cd generator
python generator.py

This creates:

stream.bin
2. Run Rust Ingestion Engine
cd ingestion-engine
cargo run
Example Output
{"tx_hash":"bf6e89eefff67189bb5222bd5a8a23e55a2428a9e05c017349be810065a84654","amount":1500,"memo":"large"}
{"tx_hash":"143a8535a14e3bfc80fc22a68a5b67ec83205e246d49c3ec1ad7e3b59c0e113f","amount":1001,"memo":"just_pass"}
Design Considerations

Python chosen for rapid binary stream simulation

Rust chosen for memory safety and performance in ingestion layer

Big-endian parsing handled explicitly

No unwrap() used on external input

Robust error handling for malformed packets
