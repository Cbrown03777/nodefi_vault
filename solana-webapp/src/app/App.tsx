// src/App.tsx
import { useEffect, useState } from 'react';
import { connection, program }           from '../../../webapp/src/solana';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import './App.css';

function App() {
  const [balance, setBalance] = useState<number | null>(null);

  useEffect(() => {
    (async () => {
      // fetch the lamport balance of the deployed program account
      const lamports = await connection.getBalance(program.programId);
      setBalance(lamports / LAMPORTS_PER_SOL);
    })();
  }, []);

  return (
    <div className="App">
      <h1>nodefi_vault on Devnet</h1>
      <p>Program ID: <code>{program.programId.toBase58()}</code></p>
      <p>
        {balance === null
          ? 'Loading balance…'
          : `Program account holds ${balance.toLocaleString()} SOL`}
      </p>
    </div>
  );
}

export default App;
