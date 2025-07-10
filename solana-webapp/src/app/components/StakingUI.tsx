import React, { useState, useEffect } from 'react';
import type { PublicKey } from '@solana/web3.js';
import { program } from '../solana';

export default function StakingUI() {
  const [accounts, setAccounts] = useState<PublicKey[]>([]);

  useEffect(() => {
    (async () => {
      try {
        // fetch all accounts of type `stakeState`
        const all = await program.account.stakeState.all();
        setAccounts(all.map((a) => a.publicKey));
      } catch (err) {
        console.error('Failed to fetch stake accounts', err);
      }
    })();
  }, []);

  return (
    <div style={{ padding: 20 }}>
      <h2>My Stake Accounts</h2>
      {accounts.length === 0 ? (
        <p>No accounts found</p>
      ) : (
        <ul>
          {accounts.map((pk) => (
            <li key={pk.toBase58()}>{pk.toBase58()}</li>
          ))}
        </ul>
      )}
    </div>
  );
}

