
<!-- Description starts here -->

A smart contract for on-demand auction of the most sought-after products on the network using the benefits of the blockchain to implement auction modalities such as Candle Auction, Dutch Auction and Sealed-bid auction.

<!-- End of description -->

This smart contracts are now live in the Vara Stabel Testne 😄T

<a style="color: #2bd071; font-size: 28px; text-decoration: none;" href="https://idea.gear-tech.io/programs/0x959d3f6039f7fedf0c2cb2c0b6fac9425f7e325c0ab1c7fb24b21fe56bca4938?node=wss%3A%2F%2Ftestnet.vara.rs">
Seal Bid Auction 🔒</a></br>

<a style="color: #2bd071; font-size: 28px; text-decoration: none;" href="https://idea.gear-tech.io/programs/0x5b49b152837b7db4cfa28a984afcf06f7874e72b9130c2bd3aaa9928bfe639d2?node=wss%3A%2F%2Ftestnet.vara.rs">
Candle Auction 🕯️</a></br>

<a style="color: #2bd071; font-size: 28px; text-decoration: none;" href="https://idea.gear-tech.io/programs/0x0ca669a405d46aa3b840f4bd79dbf0841173732dc7433907657ad641149005d3?node=wss%3A%2F%2Ftestnet.vara.rs">
Dutch Auction 🕰️</a></br>

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
