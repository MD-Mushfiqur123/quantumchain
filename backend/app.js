// QuantumChain Full-Stack Frontend
const API_URL = 'http://localhost:3000/api';
const WS_URL = 'ws://localhost:8081';

let ws = null;
let currentWallet = null;

// Console Logging
function log(message, type = 'info') {
    const console = document.getElementById('console');
    const timestamp = new Date().toLocaleTimeString();
    const line = document.createElement('div');
    line.className = `console-line ${type}`;
    line.textContent = `[${timestamp}] [${type.toUpperCase()}] ${message}`;
    console.appendChild(line);
    console.scrollTop = console.scrollHeight;
}

// API Helper
async function apiCall(endpoint, method = 'GET', body = null) {
    try {
        const options = {
            method,
            headers: {
                'Content-Type': 'application/json'
            }
        };

        if (body) {
            options.body = JSON.stringify(body);
        }

        const response = await fetch(`${API_URL}${endpoint}`, options);
        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'API request failed');
        }

        return data;
    } catch (error) {
        log(`API Error: ${error.message}`, 'error');
        throw error;
    }
}

// WebSocket Connection
function connectWebSocket() {
    try {
        ws = new WebSocket(WS_URL);

        ws.onopen = () => {
            log('✅ WebSocket connected', 'success');
            document.getElementById('apiStatus').textContent = '🟢 API Connected';
            document.getElementById('apiStatus').style.background = 'var(--success)';
        };

        ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            handleWebSocketMessage(message);
        };

        ws.onerror = (error) => {
            log('❌ WebSocket error', 'error');
        };

        ws.onclose = () => {
            log('⚠️ WebSocket disconnected. Reconnecting...', 'error');
            document.getElementById('apiStatus').textContent = '🔴 API Disconnected';
            document.getElementById('apiStatus').style.background = 'var(--danger)';
            setTimeout(connectWebSocket, 3000);
        };
    } catch (error) {
        log(`WebSocket connection failed: ${error.message}`, 'error');
    }
}

function handleWebSocketMessage(message) {
    switch (message.type) {
        case 'node_started':
            updateStats(message.data);
            break;
        case 'new_block':
            log(`📦 New block #${message.data.number} mined`, 'success');
            document.getElementById('blockHeight').textContent = message.data.number;
            break;
        case 'new_transaction':
            log(`💸 New transaction: ${message.data.hash.substring(0, 20)}...`, 'info');
            addTransactionToList(message.data);
            break;
        case 'transaction_confirmed':
            log(`✅ Transaction confirmed: ${message.data.hash.substring(0, 20)}...`, 'success');
            break;
        case 'stats_update':
            updateStats(message.data);
            break;
        case 'new_validator':
            log(`🛡️ New validator joined: ${message.data.address.substring(0, 20)}...`, 'success');
            break;
    }
}

function updateStats(stats) {
    if (stats.blockHeight !== undefined) {
        document.getElementById('blockHeight').textContent = stats.blockHeight.toLocaleString();
    }
    if (stats.tps !== undefined) {
        document.getElementById('tps').textContent = stats.tps.toLocaleString();
    }
    if (stats.validators !== undefined) {
        document.getElementById('validators').textContent = stats.validators;
    }
    if (stats.peers !== undefined) {
        document.getElementById('peers').textContent = stats.peers;
    }
}

// Tab Switching
function switchTab(tabName) {
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.classList.remove('active');
    });
    document.querySelectorAll('.tab').forEach(tab => {
        tab.classList.remove('active');
    });

    document.getElementById(tabName + '-tab').classList.add('active');
    event.target.classList.add('active');
}

// Node Control
async function startNode() {
    try {
        log('🚀 Starting QuantumChain node...', 'info');
        const result = await apiCall('/node/start', 'POST');

        document.getElementById('nodeStatus').classList.add('active');
        document.getElementById('nodeStatusText').textContent = 'Running';
        document.getElementById('stopBtn').disabled = false;

        log('✅ Node started successfully!', 'success');
        updateStats(result.stats);
    } catch (error) {
        log(`❌ Failed to start node: ${error.message}`, 'error');
    }
}

async function stopNode() {
    try {
        log('⏹️ Stopping node...', 'info');
        await apiCall('/node/stop', 'POST');

        document.getElementById('nodeStatus').classList.remove('active');
        document.getElementById('nodeStatusText').textContent = 'Stopped';
        document.getElementById('stopBtn').disabled = true;

        log('✅ Node stopped', 'success');
    } catch (error) {
        log(`❌ Failed to stop node: ${error.message}`, 'error');
    }
}

async function restartNode() {
    await stopNode();
    setTimeout(startNode, 1000);
}

// Wallet Functions
async function generateWallet() {
    try {
        log('🔑 Generating new wallet...', 'info');
        const result = await apiCall('/wallet/create', 'POST');

        currentWallet = result.wallet;
        document.getElementById('walletAddress').value = currentWallet.address;
        document.getElementById('balance').value = `${currentWallet.balance} QTM`;
        document.getElementById('privateKey').value = currentWallet.privateKey;

        log(`✅ Wallet created: ${currentWallet.address}`, 'success');
        log(`💰 Initial balance: ${currentWallet.balance} QTM`, 'info');
    } catch (error) {
        log(`❌ Failed to create wallet: ${error.message}`, 'error');
    }
}

// Transaction Functions
async function sendTransaction() {
    const to = document.getElementById('toAddress').value.trim();
    const amount = parseFloat(document.getElementById('amount').value);

    if (!currentWallet) {
        alert('⚠️ Please generate a wallet first!');
        log('❌ No wallet found', 'error');
        return;
    }

    if (!to || to.length < 10) {
        alert('⚠️ Please enter a valid recipient address!');
        return;
    }

    if (!amount || amount <= 0) {
        alert('⚠️ Please enter a valid amount!');
        return;
    }

    try {
        log(`📤 Sending ${amount} QTM to ${to}...`, 'info');

        const result = await apiCall('/transaction/send', 'POST', {
            from: currentWallet.address,
            to,
            amount,
            privateKey: currentWallet.privateKey
        });

        log(`✅ Transaction sent! Hash: ${result.transaction.hash}`, 'success');
        log(`⛽ Gas used: ${result.transaction.gasUsed}`, 'info');

        // Update balance
        const walletData = await apiCall(`/wallet/${currentWallet.address}`);
        currentWallet.balance = walletData.balance;
        document.getElementById('balance').value = `${walletData.balance} QTM`;

        // Clear form
        document.getElementById('toAddress').value = '';
        document.getElementById('amount').value = '';

        // Show success
        alert(`✅ Transaction sent successfully!\n\nHash: ${result.transaction.hash}\nAmount: ${amount} QTM`);

    } catch (error) {
        log(`❌ Transaction failed: ${error.message}`, 'error');
        alert(`❌ Transaction failed: ${error.message}`);
    }
}

function addTransactionToList(tx) {
    const txList = document.getElementById('txList');

    if (txList.children[0] && txList.children[0].textContent.includes('No transactions')) {
        txList.innerHTML = '';
    }

    const txItem = document.createElement('div');
    txItem.className = 'tx-item';
    txItem.innerHTML = `
        <div>
            <div class="tx-hash">${tx.hash.substring(0, 20)}...</div>
            <div style="font-size: 0.9rem; opacity: 0.7;">To: ${tx.to.substring(0, 20)}...</div>
            <div style="font-size: 0.8rem; opacity: 0.5; margin-top: 5px;">
                ${new Date(tx.timestamp).toLocaleString()}
            </div>
        </div>
        <div>
            <div class="tx-amount">${tx.amount} QTM</div>
            <div style="font-size: 0.8rem; opacity: 0.7; text-align: right;">
                ${tx.status === 'confirmed' ? '✅ Confirmed' : '⏳ Pending'}
            </div>
        </div>
    `;
    txList.insertBefore(txItem, txList.firstChild);
}

// Contract Functions
async function deployContract() {
    const code = document.getElementById('contractCode').value;

    if (!currentWallet) {
        alert('⚠️ Please generate a wallet first!');
        return;
    }

    if (!code) {
        alert('⚠️ Please provide contract code!');
        return;
    }

    try {
        log('🚀 Deploying smart contract...', 'info');

        const result = await apiCall('/contract/deploy', 'POST', {
            code,
            args: document.getElementById('constructorArgs').value,
            from: currentWallet.address
        });

        log(`✅ Contract deployed at: ${result.deployment.contractAddress}`, 'success');
        log(`⛽ Gas used: ${result.deployment.gasUsed}`, 'info');

        alert(`✅ Contract deployed!\n\nAddress: ${result.deployment.contractAddress}`);

    } catch (error) {
        log(`❌ Deployment failed: ${error.message}`, 'error');
        alert(`❌ Deployment failed: ${error.message}`);
    }
}

// Validator Functions
async function becomeValidator() {
    const stake = parseFloat(document.getElementById('stakeAmount').value);

    if (!currentWallet) {
        alert('⚠️ Please generate a wallet first!');
        return;
    }

    if (!stake || stake < 32) {
        alert('⚠️ Minimum stake is 32 QTM!');
        return;
    }

    try {
        log(`🛡️ Registering as validator with ${stake} QTM stake...`, 'info');

        const result = await apiCall('/validator/register', 'POST', {
            address: currentWallet.address,
            stake
        });

        log(`✅ You are now a validator! ID: ${result.validator.id}`, 'success');

        // Update balance
        const walletData = await apiCall(`/wallet/${currentWallet.address}`);
        currentWallet.balance = walletData.balance;
        document.getElementById('balance').value = `${walletData.balance} QTM`;

        alert(`✅ You are now a validator!\n\nValidator ID: ${result.validator.id}\nStake: ${stake} QTM`);

    } catch (error) {
        log(`❌ Validator registration failed: ${error.message}`, 'error');
        alert(`❌ Registration failed: ${error.message}`);
    }
}

// Quick Actions
async function viewBlocks() {
    try {
        const result = await apiCall('/blocks?limit=10');
        log(`📦 Fetched ${result.blocks.length} recent blocks`, 'info');
        console.log('Blocks:', result.blocks);
        alert(`📦 Check console for ${result.blocks.length} recent blocks`);
    } catch (error) {
        log(`❌ Failed to fetch blocks: ${error.message}`, 'error');
    }
}

async function refreshStats() {
    try {
        const result = await apiCall('/stats');
        updateStats(result.stats);
        log('🔄 Stats refreshed', 'success');
    } catch (error) {
        log(`❌ Failed to refresh stats: ${error.message}`, 'error');
    }
}

function exportKeys() {
    if (!currentWallet) {
        alert('⚠️ No wallet to export!');
        return;
    }

    const keys = {
        address: currentWallet.address,
        privateKey: currentWallet.privateKey,
        balance: currentWallet.balance
    };

    const blob = new Blob([JSON.stringify(keys, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'quantum-wallet-keys.json';
    a.click();

    log('💾 Keys exported successfully', 'success');
}

async function testAPI() {
    try {
        log('🧪 Testing API connection...', 'info');
        const result = await apiCall('/health');
        log(`✅ API Test: ${result.message}`, 'success');
        alert(`✅ API is working!\n\n${result.message}\nVersion: ${result.version}`);
    } catch (error) {
        log(`❌ API Test failed: ${error.message}`, 'error');
        alert(`❌ API is not responding!\n\nMake sure the backend server is running:\ncd backend\nnpm start`);
    }
}

// Initialize
async function init() {
    log('🚀 Initializing QuantumChain Dashboard...', 'info');

    // Connect WebSocket
    connectWebSocket();

    // Test API
    try {
        await testAPI();
    } catch (error) {
        log('⚠️ Backend API not available. Please start the server:', 'error');
        log('   cd backend && npm install && npm start', 'info');
    }

    log('✅ Dashboard ready!', 'success');
}

// Start on load
window.addEventListener('load', init);
