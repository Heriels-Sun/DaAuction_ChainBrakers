
<!-- Description starts here -->

A smart contract for on-demand auction of the most sought-after products on the network using the benefits of the blockchain to implement auction modalities such as Candle Auction, Dutch Auction and Sealed-bid auction.

<!-- End of description -->

## Building Locally

You can build locally every smart contracts in this branch, also, you can try the Front-End.

> Note: This works with every smart contract 

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```
