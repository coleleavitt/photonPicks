browser.webRequest.onBeforeSendHeaders.addListener(
    (details) => {
        details.requestHeaders.push({
            name: "Origin",
            value: "https://photon-sol.tinyastro.io"
        });
        return { requestHeaders: details.requestHeaders };
    },
    { urls: ["wss://ws-token-sol-lb.tinyastro.io/*"] },
    ["blocking", "requestHeaders"]
);