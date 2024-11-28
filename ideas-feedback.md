# Ideas

Cipher: Check for duplicate tokens by name and img

- Moicky: Note that duplicate name might not always mean scam.
  - Check duplicate images using:
    - Image hash (it might still not match even if it's the same image due to compression)
    - Image Source URL
    - Image Contents using ML? ~ .04 $ / image on OpenAI

Cipher: the one with the highest liquidity and fully diluted valuation (if market-cap is associated with this as well)

Cole: Integrate DB to track the tokens and their respective data (duplicate tokens by name and img and track statistics on them to see which is the best)
Cole: Calculate fees when making buys and sells so you don't lose profits and go negative due to that lost money

- Moicky: Hell yea!

Cole: instead of directly updating the record, create like an object with like updates like and their time that the update happened so we are tracking the coins
Cole: integrate some form of rugpull detection that can like keep an index of wallets that have been associated with Rugpull schemes and blacklist them

- Moicky: Rug Pull detection could work with the following:
  - Tracking the wallets who create those tokens. Also track what other wallets they fund over time.
  - Analyse trading patterns:
    - Volume within each 1s Candle.
    - Volume per Wallet within each Candle
    - Holders over time within the first few seconds

Cole: pump.fun addresses have pump at the end and some rugpull coins that have been identified don't have the pump at the end

- Moicky: Still some tokens blow up and allow huge profits without the `pump`-ending in the address

Cole: Have a GUI so easier access so when a coin passes rugpull check after getting inserted into the DB it will post to the GUI and have a hyperlink avaiable
Cole: have the option to do auto trading and manual trading (just giving picks)
Cole: have a risk level for rugpulls to identify what is the risk for like doing short term trades so basically the length of that coin living before I get cooked
Cole: if a coin is a cult like is legit and has legit people under it than it will last longer than meme coins that you got no clue about so we assume that all coins shall be traded in short term because they can rugpull at any time, never assume a coin will go back up as that's emotional trading
Cole: do small amount of trades per day as the higher the amount the higher the risk and volatility of your bankroll. Have an expected amount to make per day such as per hour so when we hit our profit for the day we stop automatically
Cole: have short term trades but have the ability to do like a median average over a sample size of coins and see when coins are rugging so we have a security of knowing a coin won't rug in our trades (based on our risk factor calculated)
Cole: have the ability for a demo account to test your trading strategy rather than connecting a wallet and just trading without conscience.

Edwin: Have some form of insider detection for bundled wallets that are risks of rugpulls (wallets having similar holding amounts), (% bundled)
Edwin: get statistics for wallets
Edwin: have some form of wallet tracking for (user preferred wallets such as top level traders) so whenever they make a movement on a coin we can trade given coin (copytrading)
Edwin: twitters are being reused as rugpulls reused socials by changing the bio with a different ca or even changing the image and other stuff

- Moicky: Crawl the related website links regularily to find tokens which are actually listed on the token's website
  - If a website is listed in the token's socials, but does not contain the tokens's CA -> maybed don't buy that shit?
  - Also if a website is reused for multiple tokens -> maybe don't buy that shit?

Saddam: track wallets and see which top level traders are trading the "best" coins (user provides a list of wallets they find and know are top level traders). Go through the wallets and figure out which coins they are buying

Moicky: Maybe take the opposite approach:

- Instead of tracking every token and then filtering out the rug pull tokens, why not filter all the "legit" looking tokens. (blacklist vs whitelist)
  - Would require analysis of the top tokens which blew up in the past
  - Maybe look at their volume / unique holders during the first few hours?
  - Look at social media presence e.g.:
    - look at the results when searching for the tokens name on tiktok, twitter and so on.
