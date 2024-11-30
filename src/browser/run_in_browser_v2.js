// Get the existing discover channel instance
const channel = window._discoverChannel;

// Initialize WebSocket connection to our local server
const localWs = new WebSocket('ws://localhost:8080');

localWs.onopen = () => {
    console.log('Connected to local WebSocket server');
};

localWs.onerror = (error) => {
    console.error('WebSocket error:', error);
};

localWs.onclose = () => {
    console.log('Disconnected from local WebSocket server');
};

// Subscribe to the existing channel and forward data to local WebSocket
if (channel) {
    channel.subscribe((data) => {
        if (data?.discover?.data) {
            const tokens = data.discover.data;

            // Forward to local WebSocket if connection is open
            if (localWs.readyState === WebSocket.OPEN) {
                localWs.send(JSON.stringify({
                    tokens: tokens
                }));
            }

            // console.log("Received tokens:", tokens);
        }
    });
}

// Keep connection alive
setInterval(() => {
    if (localWs.readyState === WebSocket.OPEN) {
        localWs.send(JSON.stringify({ type: 'ping' }));
    }
}, 30000);

// Reconnection logic
window.addEventListener('beforeunload', () => {
    if (localWs.readyState === WebSocket.OPEN) {
        localWs.close();
    }
});
