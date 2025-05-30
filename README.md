# Multi-Chain Crypto Wallet API

A high-performance, secure REST API for generating cryptocurrency wallets across 30+ blockchain networks. 
Built with Rust for maximum performance and security. A TrustWallet Wallet-Core alternative (trying to be)
Please support!!

## 🚀 Features

- **30+ Blockchain Support**: Generate wallets for Bitcoin, Ethereum, Solana, Cosmos ecosystem, and many more
- **BIP39 Mnemonic Generation**: Support for multiple languages (English, Japanese, Korean, Spanish, Chinese, French, Italian, Czech, Portuguese)
- **HD Wallet Support**: Hierarchical Deterministic wallet generation following BIP32/BIP44 standards
- **Symbol-Based API**: Simple, intuitive API using currency symbols (BTC, ETH, etc.)
- **Multiple Address Formats**: Automatic support for chains with multiple address types (e.g., Bitcoin Legacy/SegWit/Taproot)
- **High Performance**: Built with Rust and Actix-web for blazing-fast response times
- **Secure**: No private keys are stored; everything is generated on-the-fly
- **CORS Enabled**: Ready for web application integration

## 📋 Supported Blockchains

| Symbol | Blockchain | Address Format |
|--------|------------|----------------|
| BTC | Bitcoin | Legacy (1...), SegWit (bc1q...), Taproot (bc1p...) |
| ETH | Ethereum | 0x... |
| SOL | Solana | Base58 |
| DOT | Polkadot | SS58 (1...) |
| ADA | Cardano | addr1... |
| XRP | Ripple | r... |
| DOGE | Dogecoin | D... |
| ATOM | Cosmos | cosmos1... |
| OSMO | Osmosis | osmo1... |
| JUNO | Juno | juno1... |
| SCRT | Secret Network | secret1... |
| SEI | Sei | sei1... |
| TIA | Celestia | celestia1... |
| INJ | Injective | inj1... |
| XTZ | Tezos | tz1... |
| ALGO | Algorand | 58-char Base32 |
| NEAR | NEAR Protocol | Hex string |
| TON | Toncoin | EQ... |
| TRX | TRON | T... |
| SUI | Sui | 0x... |
| XLM | Stellar | G... |
| XMR | Monero | 4... |
| AKT | Akash | akash1... |
| EOS | EOS | EOS... |
| HBAR | Hedera | 0.0.xxxxx |
| FIL | Filecoin | f1... |
| MINA | Mina | B62... |
| ICP | Internet Computer | Principal ID |

## 🛠️ Installation

### Prerequisites

- Rust 1.75 or higher
- Cargo

### Clone and Build

```bash
git clone https://github.com/yourusername/multichain-wallet.git
cd multichain-wallet
cargo build --release
```

### Run

```bash
# Development
cargo run

# Production
./target/release/crypto-wallet-api
```

### Docker

```bash
# Build
docker build -t multichain-wallet .

# Run
docker run -p 8080:8080 multichain-wallet
```

## 📖 API Documentation

### Base URL
```
http://localhost:8080/api/v1
```

### Endpoints

#### 1. Health Check
```http
GET /health
```

Response:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": 1703123456
}
```

#### 2. Generate Mnemonic
```http
POST /mnemonic/generate
```

Request:
```json
{
  "language": "english",
  "word_count": 12
}
```

Response:
```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "language": "english",
  "word_count": 12,
  "generated_at": 1703123456
}
```

#### 3. Validate Mnemonic
```http
POST /mnemonic/validate
```

Request:
```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "language": "english"
}
```

Response:
```json
{
  "valid": true,
  "word_count": 12,
  "message": "Valid mnemonic phrase"
}
```

#### 4. Generate Wallet
```http
POST /wallet/generate
```

Request:
```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "passphrase": "",
  "index": 0,
  "symbol": "ETH"
}
```

Response (Single address for most coins):
```json
{
  "address": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",
  "chain_name": "Ethereum",
  "chain_symbol": "ETH",
  "address_type": "ethereum",
  "derivation_path": "m/44'/60'/0'/0/0",
  "index": 0,
  "public_key": "02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737",
  "private_key": "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
}
```

Response (Array for BTC):
```json
[
  {
    "address": "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
    "chain_name": "Bitcoin",
    "chain_symbol": "BTC",
    "address_type": "bitcoin_legacy",
    "derivation_path": "m/44'/0'/0'/0/0",
    "index": 0,
    "public_key": "...",
    "private_key": "..."
  },
  {
    "address": "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
    "chain_name": "Bitcoin",
    "chain_symbol": "BTC",
    "address_type": "bitcoin_segwit",
    "derivation_path": "m/84'/0'/0'/0/0",
    "index": 0,
    "public_key": "...",
    "private_key": "..."
  },
  {
    "address": "bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297",
    "chain_name": "Bitcoin",
    "chain_symbol": "BTC",
    "address_type": "bitcoin_taproot",
    "derivation_path": "m/86'/0'/0'/0/0",
    "index": 0,
    "public_key": "...",
    "private_key": "..."
  }
]
```

#### 5. Batch Generate Wallets
```http
POST /wallet/batch
```

Request:
```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "passphrase": "",
  "start_index": 0,
  "count": 5,
  "symbols": ["BTC", "ETH", "SOL"]
}
```

#### 6. Get Supported Languages
```http
GET /languages
```

#### 7. Get Supported Wallet Types
```http
GET /wallet/types
```

## 🔧 Configuration

Environment variables:
```bash
# Host and Port
APP_HOST=0.0.0.0
APP_PORT=8080

# Logging
APP_LOG_LEVEL=info
```

## 🏗️ Architecture

```
src/
├── api/
│   ├── handlers.rs    # HTTP request handlers
│   └── models.rs      # Request/Response models
├── chains/
│   ├── bitcoin.rs     # Bitcoin implementation
│   ├── ethereum.rs    # Ethereum implementation
│   ├── cosmos.rs      # Cosmos ecosystem
│   └── ...           # Other chain implementations
├── core/
│   ├── chain_info.rs  # Chain metadata
│   ├── traits.rs      # Core traits
│   └── types.rs       # Core types
├── services/
│   └── wallet.rs      # Wallet generation service
├── errors.rs          # Error handling
├── config.rs          # Configuration
└── main.rs           # Application entry point
```

## 🔐 Security Considerations

1. **No Storage**: Private keys are never stored - they're generated on-demand
2. **Secure Random**: Uses cryptographically secure random number generation
3. **Standard Compliance**: Follows BIP32/BIP39/BIP44 standards
4. **Memory Safety**: Built with Rust for memory safety guarantees


## 🤝 Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests. - will be added soon

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) for Bitcoin support
- [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) for Ed25519 curves
- [bip39](https://github.com/rust-bitcoin/rust-bip39) for mnemonic generation
- All the amazing Rust crypto libraries that make this possible

## 📞 Support

- Create an issue for bug reports or feature requests
- Will keep adding new chains

---

Made with ❤️ by the crypto community, for the crypto community.
