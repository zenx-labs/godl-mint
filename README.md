# GODL Mint

GODL Mint manages the mint authority of the GODL token and secures emissions rules.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.

## Instructions

- [`MintGODL`](program/src/mint_godl.rs) - Mints new GODL tokens to the treasury.

## State

- [`Authority`](api/src/state/authority.rs) - Holds the mint authority.
