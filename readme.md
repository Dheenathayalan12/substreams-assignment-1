solana-transfer-assignment/
│
├── src/
│   └── lib.rs
│
├── proto/
│   └── transfers.proto
│
├── substreams.yaml
├── Cargo.toml
├── buf.yaml
├── buf.gen.yaml
├── .gitignore
├── README.md
│
└── output/
    └── transfers.jsonl


# Solana Transfer Extraction

## Project Overview

This project implements a Rust-based Substreams
module to extract transfer instructions from
Solana Mainnet.

## Assignment Details

Company: CLR3 Ventures

Blockchain: Solana Mainnet

Start Block: 385870151

End Block: 385870156

## Technologies Used

- Rust
- Substreams
- Protocol Buffers
- Solana
- WebAssembly

## Features

- Processes Solana blocks.
- Extracts native SOL transfers.
- Extracts SPL Token transfers.
- Handles inner instructions.
- Resolves sender and recipient addresses.
- Produces JSON Lines output.

## Project Structure

- src/lib.rs - Rust extraction logic
- proto/transfers.proto - Output schema
- substreams.yaml - Substreams manifest
- output/transfers.jsonl - Extracted results

## Build

substreams build

## Package

substreams pack substreams.yaml

## Run

substreams run substreams.yaml map_transfers \
-e mainnet.sol.streamingfast.io:443 \
--start-block 385870151 \
--stop-block 385870157

## Output

Each transfer is represented by:

{
  "from": "SENDER_ADDRESS",
  "to": "RECIPIENT_ADDRESS",
  "amount": 325253
}
