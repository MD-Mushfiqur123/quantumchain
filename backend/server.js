const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const crypto = require('crypto');
const { v4: uuidv4 } = require('uuid');
const WebSocket = require('ws');

const app = express();
const PORT = 3000;

// Middleware
app.use(cors());
app.use(bodyParser.json());
app.use(express.static(__dirname));

// In-memory blockchain state
let blockchain = {
    blocks: [],
    transactions: [],
    pendingTransactions: [],
    wallets: new Map(),
    validators: [],
    nodeRunning: false,
    stats: {
        blockHeight: 0,
        tps: 0,
        validators: 0,
        peers: 0
    }
};

// WebSocket for real-time updates
const wss = new WebSocket.Server({ port: 8081 });
const clients = new Set();

wss.on('connection', (ws) => {
    clients.add(ws);
    console.log('📡 New WebSocket client connected');

    ws.on('close', () => {
        clients.delete(ws);
        console.log('📡 WebSocket client disconnected');
    });
});

function broadcast(data) {
    clients.forEach(client => {
        if (client.readyState === WebSocket.OPEN) {
            client.send(JSON.stringify(data));
        }
    });
}

// Helper Functions
function generateHash(data) {
    return crypto.createHash('sha256').update(JSON.stringify(data)).digest('hex');
}

function createBlock(transactions) {
    const block = {
        number: blockchain.stats.blockHeight + 1,
        timestamp: Date.now(),
        transactions: transactions,
        previousHash: blockchain.blocks.length > 0
            ? blockchain.blocks[blockchain.blocks.length - 1].hash
            : '0'.repeat(64),
        nonce: Math.floor(Math.random() * 1000000),
        validator: blockchain.validators.length > 0
            ? blockchain.validators[0].address
            : '0x0000000000000000000000000000000000000000'
    };

    block.hash = generateHash(block);
    return block;
}

// API Routes

// Health Check
app.get('/api/health', (req, res) => {
    res.json({
        status: 'ok',
        message: 'QuantumChain API Server Running',
        version: '1.0.0'
    });
});

// Node Control
app.post('/api/node/start', (req, res) => {
    if (blockchain.nodeRunning) {
        return res.status(400).json({ error: 'Node already running' });
    }

    blockchain.nodeRunning = true;

    // Create genesis block if needed
    if (blockchain.blocks.length === 0) {
        const genesisBlock = createBlock([]);
        blockchain.blocks.push(genesisBlock);
        blockchain.stats.blockHeight = 1;
    }

    // Start block production simulation
    startBlockProduction();

    broadcast({ type: 'node_started', data: blockchain.stats });

    res.json({
        success: true,
        message: 'Node started successfully',
        stats: blockchain.stats
    });
});

app.post('/api/node/stop', (req, res) => {
    blockchain.nodeRunning = false;
    stopBlockProduction();

    broadcast({ type: 'node_stopped' });

    res.json({
        success: true,
        message: 'Node stopped successfully'
    });
});

app.get('/api/node/status', (req, res) => {
    res.json({
        running: blockchain.nodeRunning,
        stats: blockchain.stats,
        blockHeight: blockchain.blocks.length,
        pendingTransactions: blockchain.pendingTransactions.length
    });
});

// Wallet Management
app.post('/api/wallet/create', (req, res) => {
    const privateKey = crypto.randomBytes(32).toString('hex');
    const address = '0x' + crypto.randomBytes(20).toString('hex');

    const wallet = {
        address,
        privateKey,
        balance: 1000, // Initial balance
        nonce: 0,
        createdAt: Date.now()
    };

    blockchain.wallets.set(address, wallet);

    res.json({
        success: true,
        wallet: {
            address: wallet.address,
            balance: wallet.balance,
            privateKey: wallet.privateKey
        }
    });
});

app.get('/api/wallet/:address', (req, res) => {
    const { address } = req.params;
    const wallet = blockchain.wallets.get(address);

    if (!wallet) {
        return res.status(404).json({ error: 'Wallet not found' });
    }

    res.json({
        address: wallet.address,
        balance: wallet.balance,
        nonce: wallet.nonce
    });
});

app.get('/api/wallet/:address/balance', (req, res) => {
    const { address } = req.params;
    const wallet = blockchain.wallets.get(address);

    if (!wallet) {
        return res.status(404).json({ error: 'Wallet not found' });
    }

    res.json({ balance: wallet.balance });
});

// Transactions
app.post('/api/transaction/send', (req, res) => {
    const { from, to, amount, privateKey } = req.body;

    if (!blockchain.nodeRunning) {
        return res.status(400).json({ error: 'Node is not running' });
    }

    const fromWallet = blockchain.wallets.get(from);

    if (!fromWallet) {
        return res.status(404).json({ error: 'Sender wallet not found' });
    }

    if (fromWallet.balance < amount) {
        return res.status(400).json({ error: 'Insufficient balance' });
    }

    // Create transaction
    const transaction = {
        hash: '0x' + crypto.randomBytes(32).toString('hex'),
        from,
        to,
        amount: parseFloat(amount),
        timestamp: Date.now(),
        nonce: fromWallet.nonce++,
        gasUsed: Math.floor(Math.random() * 50000) + 21000,
        status: 'pending'
    };

    // Update balances
    fromWallet.balance -= parseFloat(amount);

    let toWallet = blockchain.wallets.get(to);
    if (!toWallet) {
        toWallet = {
            address: to,
            balance: parseFloat(amount),
            nonce: 0,
            createdAt: Date.now()
        };
        blockchain.wallets.set(to, toWallet);
    } else {
        toWallet.balance += parseFloat(amount);
    }

    blockchain.pendingTransactions.push(transaction);
    blockchain.transactions.push(transaction);

    // Broadcast transaction
    broadcast({
        type: 'new_transaction',
        data: transaction
    });

    // Confirm transaction after delay
    setTimeout(() => {
        transaction.status = 'confirmed';
        broadcast({
            type: 'transaction_confirmed',
            data: transaction
        });
    }, 1500);

    res.json({
        success: true,
        transaction
    });
});

app.get('/api/transactions', (req, res) => {
    const limit = parseInt(req.query.limit) || 10;
    const transactions = blockchain.transactions.slice(-limit).reverse();

    res.json({ transactions });
});

app.get('/api/transaction/:hash', (req, res) => {
    const { hash } = req.params;
    const transaction = blockchain.transactions.find(tx => tx.hash === hash);

    if (!transaction) {
        return res.status(404).json({ error: 'Transaction not found' });
    }

    res.json({ transaction });
});

// Blocks
app.get('/api/blocks', (req, res) => {
    const limit = parseInt(req.query.limit) || 10;
    const blocks = blockchain.blocks.slice(-limit).reverse();

    res.json({ blocks });
});

app.get('/api/block/:number', (req, res) => {
    const number = parseInt(req.params.number);
    const block = blockchain.blocks.find(b => b.number === number);

    if (!block) {
        return res.status(404).json({ error: 'Block not found' });
    }

    res.json({ block });
});

// Validators
app.post('/api/validator/register', (req, res) => {
    const { address, stake } = req.body;

    if (stake < 32) {
        return res.status(400).json({ error: 'Minimum stake is 32 QTM' });
    }

    const wallet = blockchain.wallets.get(address);
    if (!wallet) {
        return res.status(404).json({ error: 'Wallet not found' });
    }

    if (wallet.balance < stake) {
        return res.status(400).json({ error: 'Insufficient balance' });
    }

    const validator = {
        id: blockchain.validators.length + 1,
        address,
        stake: parseFloat(stake),
        reputation: 100,
        isActive: true,
        joinedAt: Date.now()
    };

    wallet.balance -= parseFloat(stake);
    blockchain.validators.push(validator);
    blockchain.stats.validators = blockchain.validators.length;

    broadcast({
        type: 'new_validator',
        data: validator
    });

    res.json({
        success: true,
        validator
    });
});

app.get('/api/validators', (req, res) => {
    res.json({ validators: blockchain.validators });
});

// Smart Contracts
app.post('/api/contract/deploy', (req, res) => {
    const { code, args, from } = req.body;

    if (!blockchain.nodeRunning) {
        return res.status(400).json({ error: 'Node is not running' });
    }

    const contractAddress = '0x' + crypto.randomBytes(20).toString('hex');
    const deploymentHash = '0x' + crypto.randomBytes(32).toString('hex');

    const deployment = {
        hash: deploymentHash,
        contractAddress,
        deployer: from,
        timestamp: Date.now(),
        gasUsed: Math.floor(Math.random() * 2000000) + 500000,
        status: 'confirmed'
    };

    res.json({
        success: true,
        deployment
    });
});

// Stats
app.get('/api/stats', (req, res) => {
    res.json({
        stats: blockchain.stats,
        totalTransactions: blockchain.transactions.length,
        totalBlocks: blockchain.blocks.length,
        totalValidators: blockchain.validators.length,
        totalWallets: blockchain.wallets.size
    });
});

// Block Production Simulation
let blockProductionInterval;

function startBlockProduction() {
    blockProductionInterval = setInterval(() => {
        if (!blockchain.nodeRunning) return;

        // Process pending transactions
        const txToInclude = blockchain.pendingTransactions.splice(0, 10);

        if (txToInclude.length > 0 || Math.random() > 0.5) {
            const block = createBlock(txToInclude);
            blockchain.blocks.push(block);
            blockchain.stats.blockHeight = blockchain.blocks.length;

            broadcast({
                type: 'new_block',
                data: block
            });
        }

        // Update stats
        blockchain.stats.tps = Math.floor(Math.random() * 50000) + 50000;
        blockchain.stats.peers = Math.min(50, blockchain.stats.peers + Math.floor(Math.random() * 3));

        broadcast({
            type: 'stats_update',
            data: blockchain.stats
        });

    }, 3000);
}

function stopBlockProduction() {
    if (blockProductionInterval) {
        clearInterval(blockProductionInterval);
    }
}

// Start Server
app.listen(PORT, () => {
    console.log(`
╔═══════════════════════════════════════════════════════╗
║                                                       ║
║   🚀 QuantumChain Backend API Server                 ║
║                                                       ║
║   HTTP API: http://localhost:${PORT}                    ║
║   WebSocket: ws://localhost:8080                     ║
║                                                       ║
║   Status: ✅ Running                                  ║
║                                                       ║
╚═══════════════════════════════════════════════════════╝
    `);

    console.log('📡 API Endpoints:');
    console.log('   GET  /api/health');
    console.log('   POST /api/node/start');
    console.log('   POST /api/node/stop');
    console.log('   GET  /api/node/status');
    console.log('   POST /api/wallet/create');
    console.log('   GET  /api/wallet/:address');
    console.log('   POST /api/transaction/send');
    console.log('   GET  /api/transactions');
    console.log('   GET  /api/blocks');
    console.log('   POST /api/validator/register');
    console.log('   POST /api/contract/deploy');
    console.log('   GET  /api/stats');
    console.log('');
});
