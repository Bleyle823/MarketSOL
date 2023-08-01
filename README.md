## MARKETSOL DESCRIPTION 💱
MarketSOL is a stock prediction Dapp utilizing the Pyth network to deliver real time on chain market data on stocks like Google, Tesla and crypto. Furthermore, users can bet against each other on the price of stocks and win crypto. 

On making a bet, the user can sign a transaction which then sends the money to a third party account which acts as an escrow that will then distribute the prize to the winner of the bet.

Apart from peer to peer betting, users can also speculatively vote on companies that they are bullish on. Durable Nonces is leveraged here to enable voters to vote for a given set of times, and once the time comes for counting, the votes are counted, the count is publicly announced to everyone, and the winner is declared. Instead of signing and sending the transaction when voting for your candidate, the dapp can let the user sign the transaction using durable nonces, serialize the transaction as shown above in the web3.js example, and save the serialized transactions in a database until the time comes for counting.
For counting the votes, the dapp then needs to sync, send or submit all the signed transactions one by one. With each submitted transaction, the state change will happen on-chain, and the winner can be decided.

## Getting Started
First, install the dependencies

```bash
npm install
```

run the development server:
 
```bash
npm run dev
# or
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `pages/index.js`. The page auto-updates as you edit the file.

[API routes](https://nextjs.org/docs/api-routes/introduction) can be accessed on [http://localhost:3000/api/hello](http://localhost:3000/api/hello). This endpoint can be edited in `pages/api/hello.js`.

The `pages/api` directory is mapped to `/api/*`. Files in this directory are treated as [API routes](https://nextjs.org/docs/api-routes/introduction) instead of React pages.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details..
