class WebSocketManager {
    constructor(url, options = {}) {
        this.url = url;
        this.options = {
            reconnectInterval: 5000,
            maxReconnectAttempts: 10,
            ...options
        };
        this.reconnectAttempts = 0;
        this.isConnected = false;
        this.ws = null;
        this.connect();
    }

    connect() {
        try {
            this.ws = new WebSocket(this.url);
            this.setupEventHandlers();
        } catch (error) {
            console.error(`Connection failed: ${error.message}`);
            this.scheduleReconnect();
        }
    }

    setupEventHandlers() {
        this.ws.onopen = () => {
            console.log(`Connected to ${this.url}`);
            this.isConnected = true;
            this.reconnectAttempts = 0;

            if (this.options.onOpen) {
                this.options.onOpen();
            }
        };

        this.ws.onclose = () => {
            console.log(`Connection to ${this.url} closed`);
            this.isConnected = false;
            this.scheduleReconnect();
        };

        this.ws.onerror = (error) => {
            console.error(`WebSocket error: ${error.message}`);
            this.isConnected = false;
        };

        if (this.options.onMessage) {
            this.ws.onmessage = this.options.onMessage;
        }
    }

    scheduleReconnect() {
        if (this.reconnectAttempts < this.options.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`Reconnecting... Attempt ${this.reconnectAttempts}`);
            setTimeout(() => this.connect(), this.options.reconnectInterval);
        } else {
            console.error('Max reconnection attempts reached');
        }
    }

    send(data) {
        if (this.isConnected) {
            this.ws.send(typeof data === 'string' ? data : JSON.stringify(data));
        }
    }
}

// Usage example:
const localSocket = new WebSocketManager('ws://localhost:8080', {
    onMessage: (event) => {
        console.log('Local message received:', event.data);
    }
});

const remoteSocket = new WebSocketManager('wss://ws-token-sol-lb.tinyastro.io/cable', {
    onOpen: () => {
        // Subscribe to channel on connection
        remoteSocket.send({
            command: "subscribe",
            identifier: JSON.stringify({ channel: "DiscoverLpChannel" })
        });
    },
    onMessage: (event) => {
        const data = JSON.parse(event.data);
        if (data.message?.discover?.data && localSocket.isConnected) {
            localSocket.send({
                type: 'token_data',
                data: data.message.discover.data
            });
        }
    }
});

// Keep-alive ping
setInterval(() => {
    if (remoteSocket.isConnected) {
        remoteSocket.send({ type: 'ping' });
    }
}, 30000);

// Status monitoring
setInterval(() => {
    console.log(`Status - Local: ${localSocket.isConnected}, Remote: ${remoteSocket.isConnected}`);
}, 60000);