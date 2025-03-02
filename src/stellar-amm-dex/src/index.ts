import { Asset } from '@stellar/stellar-sdk';
import { StellarAMM } from './amm';
import dotenv from 'dotenv';

dotenv.config();

const run = async () => {
  const amm = new StellarAMM();

  // Example: Create a liquidity pool for XLM and a token (e.g., USDC)
  const usdc = new Asset(
    'USDC',
    'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN' // Testnet issuer
  );

  const xlm = Asset.native();

  // Step 1: Create a liquidity pool
  const poolId = await amm.createLiquidityPool(xlm, usdc, 30);
  console.log('Liquidity Pool Created:', poolId);

  // Step 2: Add liquidity
  const depositTx = await amm.addLiquidity(poolId, '1000', '500', '0.5', '2.0');
  console.log('Liquidity Added:', depositTx);

  // Step 3: Swap XLM for USDC
  const swapTx = await amm.swap(xlm, '10', usdc, '4.8');
  console.log('Swap Completed:', swapTx);
};

run().catch(console.error);
