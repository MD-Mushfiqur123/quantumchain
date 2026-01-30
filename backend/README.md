# QuantumChain Backend API

Full-stack blockchain backend with REST API and WebSocket support.

## 🚀 Quick Start

### Install Dependencies
```bash
cd backend
npm install
```

### Start Server
```bash
npm start
```

Server will run on:
- **HTTP API**: http://localhost:3000
- **WebSocket**: ws://localhost:8080

## 📡 API Endpoints

### Node Control
- `POST /api/node/start` - Start blockchain node
- `POST /api/node/stop` - Stop blockchain node
- `GET /api/node/status` - Get node status

### Wallet Management
- `POST /api/wallet/create` - Create new wallet
- `GET /api/wallet/:address` - Get wallet info
- `GET /api/wallet/:address/balance` - Get balance

### Transactions
- `POST /api/transaction/send` - Send transaction
- `GET /api/transactions` - Get recent transactions
- `GET /api/transaction/:hash` - Get transaction by hash

### Blocks
- `GET /api/blocks` - Get recent blocks
- `GET /api/block/:number` - Get block by number

### Validators
- `POST /api/validator/register` - Register as validator
- `GET /api/validators` - Get all validators

### Smart Contracts
- `POST /api/contract/deploy` - Deploy contract

### Stats
- `GET /api/stats` - Get blockchain statistics
- `GET /api/health` - Health check

## 🌐 WebSocket Events

### Received Events
- `node_started` - Node started
- `node_stopped` - Node stopped
- `new_block` - New block mined
- `new_transaction` - New transaction
- `transaction_confirmed` - Transaction confirmed
- `stats_update` - Stats updated
- `new_validator` - New validator joined

## 🎮 Usage

1. Start backend server
2. Open `index.html` in browser
3. Click "Start Node"
4. Generate wallet
5. Send transactions!

## 📦 Dependencies

- express - Web framework
- cors - CORS middleware
- body-parser - JSON parsing
- ws - WebSocket server
- uuid - Unique IDs
- crypto - Cryptography

## 🔧 Development

```bash
npm run dev  # Start with nodemon (auto-reload)
```
