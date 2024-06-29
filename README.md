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

### Configuration

The keeper is configured using environment variables.

| Name                                    | Description                                              |
| --------------------------------------- | -------------------------------------------------------- |
| `KEEPER_RPC_URL`                        | The RPC URL of the Starket node.                         |
| `KEEPER_SIGNER_PRIVATE_KEY`             | The private key controlling the keeper account contract. |
| `KEEPER_ACCOUNT_ADDRESS`                | The address of the account contract of the keeper.       |
| `KEEPER_SATORU_EXCHANGE_ROUTER_ADDRESS` | The address of the Satoru exchange router contract.      |

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

## 📚 Resources

Here are some resources to help you get started:

- [Satoru Book](https://keep-starknet-strange.github.io/satoru/)
- [Starknet Book](https://book.starknet.io/)
- GMX v2 resources
  - [GMX Synthetics](https://github.com/gmx-io/gmx-synthetics)
  - [Trading on v2](https://docs.gmx.io/docs/trading/v2)
  - [Contracts for v2](https://docs.gmx.io/docs/api/contracts-v2/)
  - [Liquidity on v2](https://docs.gmx.io/docs/providing-liquidity/v2)
- Some DeFi offchain services example implementations
  - [swaps-offchain-infra](https://github.com/mycelium-ethereum/swaps-offchain-infra)
  - [swaps-liquidator](https://github.com/mycelium-ethereum/swaps-liquidator)

## 🫶 Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/abdelhamidbakhta"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare "/><br /><sub><b>Abdel @ StarkWare </b></sub></a><br /><a href="https://github.com/keep-starknet-strange/satoru-keeper/commits?author=abdelhamidbakhta" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://t.me/notaihe"><img src="https://avatars.githubusercontent.com/u/22559023?v=4?s=100" width="100px;" alt="akhercha"/><br /><sub><b>akhercha</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/satoru-keeper/commits?author=akhercha" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/zarboq"><img src="https://avatars.githubusercontent.com/u/37303126?v=4?s=100" width="100px;" alt="zarboq"/><br /><sub><b>akhercha</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/satoru-keeper/commits?author=zarboq" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/bacharif"><img src="https://avatars.githubusercontent.com/u/22233193?v=4?s=100" width="100px;" alt="bacharif"/><br /><sub><b>akhercha</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/satoru-keeper/commits?author=bacharif" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
