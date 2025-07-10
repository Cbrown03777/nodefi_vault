'use client';

import { useMemo } from 'react';
import { clusterApiUrl, Connection } from '@solana/web3.js';
import {
  ConnectionProvider,
  WalletProvider
} from '@solana/wallet-adapter-react';
import {
  WalletModalProvider,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';

// Update the import path below if the actual path is different, e.g.:
import StakingUI from './components/StakingUI';                    // your global styles
import '@solana/wallet-adapter-react-ui/styles.css'; // wallet-adapter styles

export default function Page() {
  // 1) pick the cluster
  const endpoint = useMemo(() => clusterApiUrl('devnet'), []);
  // 2) configure the wallets you want to support
  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div style={{ padding: '2rem' }}>
            {/* connect / disconnect button */}
            <WalletMultiButton />

            {/* your staking UI component */}
            <StakingUI />
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
