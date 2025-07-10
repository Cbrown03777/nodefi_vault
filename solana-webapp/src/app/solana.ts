// webapp/src/solana.ts
import { AnchorProvider, Program, Idl, Provider } from "@project-serum/anchor";
import { PublicKey, Connection }                 from "@solana/web3.js";
import idlJson                                  from "./target/idl/nodefi_vault.json";

// 1) Tell TS that your JSON is in fact an Anchor IDL
const idl = idlJson as unknown as Idl;

// 2) Your network & provider
export const network   = "https://api.devnet.solana.com";
export const connection = new Connection(network, "confirmed");
const provider: Provider = AnchorProvider.local();  // or AnchorProvider.env()

// 3) Your program IDL + on-chain program handle
export const PROGRAM_ID = new PublicKey(
  "F14yVkHb8S1ByEStSshM1Bep6DkFpdA9yiUkgNcs9r1V"
);
export const program    = new Program(idl, PROGRAM_ID, provider);
