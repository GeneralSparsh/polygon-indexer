# 🌌 Polygon POL Token Real-Time Indexer

A high-performance real-time blockchain data indexer built in Rust that tracks POL token transfers on Polygon blockchain, specifically monitoring net flows to Binance addresses.

## 🌟 Features

- **Real-time POL token transfer tracking** on Polygon blockchain
- **Binance address monitoring** with cumulative net-flow calculations
- **Beautiful dark galaxy-themed UI** with starfield animations and neon accents
- **High-performance Rust backend** with SQLite database
- **WebSocket real-time updates** for live data streaming
- **RESTful API** for querying indexed data
- **Scalable architecture** designed for multi-exchange support

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- SQLite development libraries
- Node.js 18+ (for frontend dependencies)

### Installation

1. **Extract the project:**
```bash
unzip polygon-indexer.zip
cd polygon-indexer
```

2. **Install Rust dependencies:**
```bash
cargo build --release
```

3. **Set up environment variables:**
```bash
cp .env.example .env
# Edit .env with your Polygon RPC settings
```

4. **Run database migrations:**
```bash
cargo run --bin setup_db
```

5. **Build the frontend:**
```bash
cd ui
npm install
npm run build
cd ..
```

6. **Start the indexer:**
```bash
cargo run --release
```

7. **Access the web interface:**
```
Open: http://localhost:3000
```

## 🎯 Tracked Binance Addresses

- `0xF977814e90dA44bFA03b6295A0616a897441aceC`
- `0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245`
- `0x505e71695E9bc45943c58adEC1650577BcA68fD9`
- `0x290275e3db66394C52272398959845170E4DCb88`
- `0xD5C08681719445A5Fdce2Bda98b341A49050d821`
- `0x082489A616aB4D46d1947eE3F912e080815b08DA`

## 📊 API Endpoints

### REST API
- `GET /api/netflow` - Get current cumulative net flow
- `GET /api/transfers` - List recent POL transfers  
- `GET /api/stats` - Get indexer statistics
- `GET /api/health` - Health check

### WebSocket
- `ws://localhost:3000/ws` - Real-time updates stream

## 🔧 Configuration

Key environment variables in `.env`:

```bash
POLYGON_RPC_URL=https://polygon-rpc.com/
POLYGON_WS_URL=wss://rpc-mainnet.matic.network
DATABASE_URL=data/indexer.db
HOST=127.0.0.1
PORT=3000
POL_CONTRACT=0x0000000000000000000000000000000000001010
RUST_LOG=info
```

## 📈 Scalability Strategy

The architecture is designed for easy expansion to support multiple exchanges and blockchains while maintaining high performance and reliability.

## 🧪 Development

### Running Tests
```bash
cargo test
```

### Database Management
```bash
cargo run --bin reset_db  # Reset database
cargo run --bin setup_db  # Setup database
```

### Frontend Development
```bash
cd ui
npm run dev    # Start development server
npm run build  # Build for production
```

## 📝 License

MIT License - see LICENSE file for details.

Built using Rust and modern web technologies
