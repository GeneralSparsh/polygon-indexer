// Polygon POL Indexer - Frontend JavaScript
class PolygonIndexer {
    constructor() {
        this.isConnected = true;
        this.init();
    }

    init() {
        this.updateConnectionStatus();
        this.startSimulation();
        this.setupEventListeners();
    }

    updateConnectionStatus() {
        const statusElement = document.getElementById('connectionStatus');
        const statusDot = statusElement.querySelector('.status-dot');
        const statusText = statusElement.querySelector('span');

        statusText.textContent = this.isConnected ? 'Connected to Polygon' : 'Disconnected';
        statusDot.className = this.isConnected ? 'status-dot connected' : 'status-dot';
    }

    startSimulation() {
        // Simulate real-time updates every 30 seconds
        setInterval(() => {
            this.updateMetrics();
            this.addRandomTransfer();
        }, 30000);

        // Update timestamp every second
        setInterval(() => {
            this.updateTimestamp();
        }, 1000);
    }

    updateMetrics() {
        // Simulate small changes in net flow
        const netFlowElement = document.getElementById('totalNetFlow');
        const currentValue = parseFloat(netFlowElement.textContent.replace(/[^0-9.-]/g, ''));
        const change = (Math.random() - 0.5) * 100000; // Random change ±50K
        const newValue = currentValue + change;

        netFlowElement.textContent = this.formatTokenAmount(newValue);
    }

    addRandomTransfer() {
        const transferList = document.getElementById('transferList');
        const addresses = [
            '0xF977814e90dA44bFA03b6295A0616a897441aceC',
            '0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245',
            '0x505e71695E9bc45943c58adEC1650577BcA68fD9'
        ];

        const randomFrom = '0x' + Math.random().toString(16).substr(2, 40);
        const randomTo = addresses[Math.floor(Math.random() * addresses.length)];
        const amount = Math.floor(Math.random() * 100000) + 1000;

        const transferItem = document.createElement('div');
        transferItem.className = 'transfer-item fade-in';
        transferItem.innerHTML = `
            <div class="transfer-icon">⬇️</div>
            <div class="transfer-details">
                <div class="transfer-hash">${this.shortenHash('0x' + Math.random().toString(16).substr(2, 64))}</div>
                <div class="transfer-addresses">
                    <span class="from">${this.shortenAddress(randomFrom)}</span>
                    ➡️
                    <span class="to">${this.shortenAddress(randomTo)}</span>
                </div>
            </div>
            <div class="transfer-value">${this.formatTokenAmount(amount)}</div>
            <div class="transfer-time">just now</div>
        `;

        transferList.insertBefore(transferItem, transferList.firstChild);

        // Remove old transfers
        const items = transferList.querySelectorAll('.transfer-item');
        if (items.length > 20) {
            items[items.length - 1].remove();
        }
    }

    updateTimestamp() {
        const timestampElement = document.getElementById('lastUpdate');
        timestampElement.textContent = new Date().toLocaleString('en-US', {
            timeZone: 'UTC',
            year: 'numeric',
            month: 'short',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit'
        }) + ' UTC';
    }

    setupEventListeners() {
        // Add any interactive features here
        document.addEventListener('visibilitychange', () => {
            if (!document.hidden) {
                this.updateMetrics();
            }
        });
    }

    formatTokenAmount(amount) {
        const value = Math.abs(amount);
        const sign = amount >= 0 ? '+' : '-';

        if (value < 1000) return sign + value.toFixed(0) + ' POL';
        if (value < 1000000) return sign + (value / 1000).toFixed(1) + 'K POL';
        if (value < 1000000000) return sign + (value / 1000000).toFixed(2) + 'M POL';
        return sign + (value / 1000000000).toFixed(2) + 'B POL';
    }

    shortenAddress(address) {
        return address.slice(0, 6) + '...' + address.slice(-4);
    }

    shortenHash(hash) {
        return hash.slice(0, 10) + '...' + hash.slice(-8);
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new PolygonIndexer();
});
