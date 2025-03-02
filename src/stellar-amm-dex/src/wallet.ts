import { Wallet } from '@stellar/typescript-wallet-sdk';
import { Networks, Transaction } from '@stellar/stellar-sdk';
import dotenv from 'dotenv';

dotenv.config();

export class StellarWallet {
  private wallet: Wallet;

  constructor() {
    this.wallet = new Wallet({
      network: process.env.STELLAR_NETWORK === 'TESTNET' ? Networks.TESTNET : Networks.PUBLIC,
      // Use Freighter or other wallet providers
      walletConnect: {
        onConnect: async (publicKey: string) => {
          console.log('Connected to wallet:', publicKey);
        },
      },
    });
  }

  async connect(): Promise<string> {
    const { publicKey } = await this.wallet.connect();
    return publicKey;
  }

  async signTransaction(tx: Transaction): Promise<Transaction> {
    return this.wallet.signTransaction(tx);
  }
}
