const ws = new WebSocket("wss://ws-token-sol-lb.tinyastro.io/cable");

// Filter constants
const MIN_MARKET_CAP = 40000;  // $40k
const MAX_MARKET_CAP = 500000; // $500k
const MAX_TOP_HOLDERS_PERC = 25; // 25% maximum for top holders
const MIN_BUY_SELL_RATIO = 1.2; // Minimum buy/sell ratio for positive trend
const MIN_VOLUME = 5000; // Minimum volume in USD
const MIN_POOLED_SOL = 20; // Minimum pooled SOL

// Function to check for positive momentum
const hasPositiveMomentum = (attributes) => {
    // Calculate key metrics
    const buyToSellRatio = attributes.buys_count / (attributes.sells_count || 1);
    const volumeToMcapRatio = attributes.volume / attributes.fdv;

    return (
        buyToSellRatio >= MIN_BUY_SELL_RATIO && // More buys than sells
        attributes.volume >= MIN_VOLUME && // Decent volume
        attributes.pooled_sol >= MIN_POOLED_SOL && // Good liquidity
        volumeToMcapRatio > 0.1 // Active trading relative to market cap
    );
};

ws.onopen = () => {
    const subscribeMessage = {
        command: "subscribe",
        identifier: JSON.stringify({channel: "DiscoverLpChannel"})
    };
    ws.send(JSON.stringify(subscribeMessage));
    console.log("Connected and subscribed to WebSocket");
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);

    if (data.message?.discover?.data) {
        const tokens = data.message.discover.data;

        tokens.forEach(token => {
            const attributes = token.attributes;
            const marketCap = attributes.fdv;
            const topHoldersPerc = attributes.audit?.top_holders_perc || 100;

            if (attributes?.socials?.twitter &&
                marketCap >= MIN_MARKET_CAP &&
                marketCap <= MAX_MARKET_CAP &&
                topHoldersPerc <= MAX_TOP_HOLDERS_PERC &&
                hasPositiveMomentum(attributes)) {

                console.log(`🚀 Trending Token Found:`);
                console.table({
                    name: attributes.name,
                    symbol: attributes.symbol,
                    twitter: attributes.socials.twitter,
                    price_usd: attributes.price_usd,
                    market_cap: marketCap.toFixed(2),
                    holders: attributes.holders_count,
                    top_holders_perc: `${topHoldersPerc.toFixed(2)}%`,
                    volume: attributes.volume,
                    volume_mcap_ratio: (attributes.volume / marketCap).toFixed(3),
                    tokenAddress: attributes.tokenAddress,
                    dev_holding: `${attributes.dev_holding_perc}%`,
                    lp_burned: `${attributes.audit?.lp_burned_perc || 0}%`
                });

                // Momentum metrics
                console.log(`Momentum Metrics:`);
                console.table({
                    buys: attributes.buys_count,
                    sells: attributes.sells_count,
                    buy_sell_ratio: (attributes.buys_count / (attributes.sells_count || 1)).toFixed(2),
                    pooled_sol: attributes.pooled_sol,
                    volume_24h: attributes.volume,
                    created_time: new Date(attributes.created_timestamp * 1000).toLocaleString(),
                    age_hours: ((Date.now() / 1000 - attributes.created_timestamp) / 3600).toFixed(2)
                });
            }
        });
    }
};

ws.onerror = (error) => {
    console.error("WebSocket error:", error);
};

// Keep connection alive
setInterval(() => {
    if (ws.readyState === ws.OPEN) {
        ws.send(JSON.stringify({type: 'ping'}));
    }
}, 30000);

// Reconnection logic
ws.onclose = () => {
    console.log("Connection closed. Attempting to reconnect...");
    setTimeout(() => {
        location.reload();
    }, 1000);
};
