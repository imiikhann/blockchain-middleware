use std::fs::File;
use std::io::{BufReader, Read};
use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;

#[derive(Serialize)]
struct Transaction {
    tx_hash: String,
    amount: u64,
    memo: String,
}

fn xor_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc ^ b)
}

fn main() {
    let file = File::open("../generator/stream.bin")
        .expect("stream.bin not found");

    let mut reader = BufReader::new(file);

    loop {
        let mut magic = [0u8; 1];
        if reader.read_exact(&mut magic).is_err() {
            break;
        }

        if magic[0] != 0xA5 {
            println!("Invalid magic byte");
            continue;
        }

        let packet_type = reader.read_u8().unwrap();
        let payload_len = reader.read_u16::<BigEndian>().unwrap() as usize;

        let mut payload = vec![0u8; payload_len];
        if reader.read_exact(&mut payload).is_err() {
            break;
        }

        let checksum = reader.read_u8().unwrap();

        if xor_checksum(&payload) != checksum {
            println!("Invalid checksum");
            continue;
        }

        // Only handle Transaction packets
        if packet_type == 0x01 && payload.len() >= 40 {
            let tx_hash = hex::encode(&payload[0..32]);
            let amount = (&payload[32..40])
                .read_u64::<BigEndian>()
                .unwrap();
            let memo = String::from_utf8_lossy(&payload[40..]).to_string();

            if amount > 1000 {
                let tx = Transaction { tx_hash, amount, memo };
                println!("{}", serde_json::to_string(&tx).unwrap());
            }
        }
    }
}