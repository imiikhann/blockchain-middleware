lockchain Middleware – Ingestion Engine
Overview

This project demonstrates a binary protocol ingestion system:

Python generator creates a binary stream (stream.bin)

Rust engine parses and processes the stream

Valid transactions with amount > 1000 are emitted as JSON

Protocol Format
Field	Size
Magic	1 byte (0xA5)
Type	1 byte
Length	2 bytes (big endian)
Payload	variable
Checksum	1 byte (XOR of payload)
Packet Types

0x01 → Transaction

0x02 → State Update

0xFF → Keep Alive

Edge Cases Handled

Invalid checksum

Invalid magic byte

Truncated packets

Unsupported packet types

How To Run
1. Generate Stream
python generator.py
2. Run Engine
cargo run
Output

Only transactions where:

amount > 1000

are emitted as JSON.
