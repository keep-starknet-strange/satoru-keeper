# Satoru keeper service 🦀

## 📝 Description

The keeper is an offchain service for Satoru protocol. It is responsible for:

- Watching the user initiated actions and execute them onchain, following the 2-steps process mechanism of GMX v2.

## 📦 Installation

### 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### 🛠️ Build

```bash
cargo build --release
```

## Usage

```bash
RUST_LOG=info cargo run
```

## As library

```rust
#[tokio::main]
async fn main() {
    let config = KeeperConfigBuilder::default()
        .rpc_url("https://127.0.0.1:5050")
        .signer_private_key("0x...")
        .account_address("0x...")
        .build()?;
    let keeper = Keeper::new(config).await.unwrap();

    // Then you can use the keeper to execute actions.
    // keeper.execute_deposit(...);
}
```

## 📄 License

This project is licensed under the MIT license.

See [LICENSE](LICENSE) for more information.

Happy coding! 🎉

## 🫶 Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/abdelhamidbakhta"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare "/><br /><sub><b>Abdel @ StarkWare </b></sub></a><br /><a href="https://github.com/keep-starknet-strange/satoru-keeper/commits?author=abdelhamidbakhta" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
