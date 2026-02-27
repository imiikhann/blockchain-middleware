import struct
import os

OUTPUT_FILE = "stream.bin"

# XOR checksum


def xor_checksum(data: bytes) -> int:
    csum = 0
    for b in data:
        csum ^= b
    return csum

# Build full packet


def build_packet(packet_type: int, payload: bytes) -> bytes:
    payload_len = len(payload)
    checksum = xor_checksum(payload)
    return struct.pack(">BBH", 0xA5, packet_type, payload_len) + payload + struct.pack("B", checksum)

# Transaction packet


def transaction(amount: int, memo: str) -> bytes:
    tx_hash = os.urandom(32)
    payload = tx_hash + struct.pack(">Q", amount) + memo.encode()
    return build_packet(0x01, payload)

# State update packet


def state_update(status: int) -> bytes:
    block_hash = os.urandom(32)
    payload = block_hash + struct.pack("B", status)
    return build_packet(0x02, payload)

# Keep alive packet


def keep_alive() -> bytes:
    payload = b"deprecated"
    return build_packet(0xFF, payload)


# Create file
with open(OUTPUT_FILE, "wb") as f:
    # Transactions
    f.write(transaction(500, "small"))      # filtered out
    f.write(transaction(1500, "large"))     # included
    f.write(transaction(1001, "just_pass"))  # included

    # Other packets
    f.write(state_update(1))
    f.write(keep_alive())

print("stream.bin generated successfully!")
