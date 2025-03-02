import { Server, Asset, TransactionBuilder, Operation, Networks } from '@stellar/stellar-sdk';
import { StellarWallet } from './wallet';
import dotenv from 'dotenv';

dotenv.config();

const server = new Server(process.env.HORIZON_URL!);

export class StellarAMM {
  private wallet: StellarWallet;

  constructor() {
    this.wallet = new StellarWallet();
  }

  // Create a liquidity pool for two assets (e.g., XLM and a token)
  async createLiquidityPool(
    assetA: Asset,
    assetB: Asset,
    feeBp: number = 30 // 0.3% fee
  ): Promise<string> {
    const publicKey = await this.wallet.connect();

    const account = await server.loadAccount(publicKey);
    const tx = new TransactionBuilder(account, {
      fee: '100',
      networkPassphrase: process.env.STELLAR_NETWORK === 'TESTNET' ? Networks.TESTNET : Networks.PUBLIC,
    })
      .addOperation(
        Operation.createLiquidityPool({
          source: publicKey,
          assetA,
          assetB,
          feeBp,
        })
      )
      .setTimeout(30)
      .build();

    const signedTx = await this.wallet.signTransaction(tx);
    const result = await server.submitTransaction(signedTx);
    return result.hash;
  }

  // Add liquidity to a pool
  async addLiquidity(
    poolId: string,
    maxAmountA: string,
    maxAmountB: string,
    minPrice: string,
    maxPrice: string
  ): Promise<string> {
    const publicKey = await this.wallet.connect();

    const account = await server.loadAccount(publicKey);
    const tx = new TransactionBuilder(account, {
      fee: '100',
      networkPassphrase: process.env.STELLAR_NETWORK === 'TESTNET' ? Networks.TESTNET : Networks.PUBLIC,
    })
      .addOperation(
        Operation.liquidityPoolDeposit({
          source: publicKey,
          poolId,
          maxAmountA,
          maxAmountB,
          minPrice,
          maxPrice,
        })
      )
      .setTimeout(30)
      .build();

    const signedTx = await this.wallet.signTransaction(tx);
    const result = await server.submitTransaction(signedTx);
    return result.hash;
  }

  // Swap assets using the AMM
  async swap(
    sendAsset: Asset,
    sendAmount: string,
    receiveAsset: Asset,
    minReceiveAmount: string
  ): Promise<string> {
    const publicKey = await this.wallet.connect();

    const account = await server.loadAccount(publicKey);
    const tx = new TransactionBuilder(account, {
      fee: '100',
      networkPassphrase: process.env.STELLAR_NETWORK === 'TESTNET' ? Networks.TESTNET : Networks.PUBLIC,
    })
      .addOperation(
        Operation.pathPaymentStrictSend({
          source: publicKey,
          sendAsset,
          sendAmount,
          destAsset: receiveAsset,
          destMin: minReceiveAmount,
          path: [], // AMM will auto-route through the pool
        })
      )
      .setTimeout(30)
      .build();

    const signedTx = await this.wallet.signTransaction(tx);
    const result = await server.submitTransaction(signedTx);
    return result.hash;
  }
}
