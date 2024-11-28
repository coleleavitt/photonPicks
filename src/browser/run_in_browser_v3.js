// First import or define ActionCableObserver
const ActionCableObserver = window.ActionCableObserver || {};

// Modified monitoring code
const monitorAllChannels = () => {
    if (!ActionCableObserver.prototype) {
        console.error('ActionCableObserver not loaded');
        return;
    }

    const originalSubscribe = ActionCableObserver.prototype.subscribe;
    ActionCableObserver.prototype.subscribe = function(callback) {
        return originalSubscribe.call(this, (resp) => {
            console.log(`Channel ${this.channelId} Response:`, resp);
            if (callback) callback(resp);
        });
    }
}

// Alternative direct channel monitoring
const monitorChannel = (channelName) => {
    const channel = window[`_${channelName}Channel`];
    if (channel && channel.subscribe) {
        channel.subscribe((resp) => {
            console.log(`${channelName} Channel Response:`, resp);
        });
    }
}

// Monitor specific channels
monitorChannel('plPrice');
monitorChannel('taxFee');
monitorChannel('watch');
monitorChannel('orders');
monitorChannel('taxFee');
monitorChannel('jito');
monitorChannel('discover');
monitorChannel('user');
monitorChannel('show');
monitorChannel('gas');
monitorChannel('broadcast');
monitorChannel('request')
