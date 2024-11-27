// Get the existing channel instance
const channel = window._discoverChannel;

// Add message handler using subscribe method
if (channel) {
    channel.subscribe((data) => {
        if (data?.discover?.data) {
            const tokens = data.discover.data;
            console.log("Received tokens:", tokens);
        }
    });
}