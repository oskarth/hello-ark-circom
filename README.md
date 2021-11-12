# Playing around with ark-circom

## Goal

Use Circom from Rust.

Specifically, witness generation, proving and rust. Then wrap Rust in Nim interface for integration into nim-waku (e.g.).

Ideally with Circom 2.0.

## Notes

Circom 1.0 seems to work for test circuits. Need to (a) try complex circuits like RLN (b) integrate with Nim.

Circom 2.0 compiled circuits doesn't yet seem to be supported. Started basic fork here with circom 2 feature flag to see how much work it'd be.

Current status: some errors I'm not sure about, and hacky approach regardless. https://github.com/oskarth/ark-circom

## Running

Change to Circom 2 by changing circuits being passed to config in `main.rs`, and using feature flag in `Cargo.toml` to enable circom-2 WIP compiler flag feature.
