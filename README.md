# ckb-miner-avx512

An avx512 miner, can run on CPUs which supporting avx512 instruction set.

## How to build
if you want build an avx2 version, which can run on most CPUs, you should add this option:

```
cargo build --release --features avx2
```

## How to use

1. run a ckb client

2. copy the ckb-miner.toml to your work directory and modify it.

3. run ckb-miner in your work directory.

```
./ckb-miner
```