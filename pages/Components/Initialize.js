import {
    useConnection,
    useWallet,
    useAnchorWallet,
  } from "@solana/wallet-adapter-react";
  import * as anchor from "@project-serum/anchor";
  import React, { useEffect, useState } from "react";
  import idl from "../idl.json";
  import { Button } from "@chakra-ui/react";
  
  const PROGRAM_ID = new anchor.web3.PublicKey(
    `8zhJi5UjxpB9HTdjkqcUrdKKJPT5yRML7jJgyKj2RPZk`
  );
  
  export const Initialize = ({ setCounter, setTransactionUrl }) => {
    const [program, setProgram] = useState(null);
  
    const { connection } = useConnection();
    const wallet = useAnchorWallet();
  
    useEffect(() => {
      let provider;
  
      try {
        provider = anchor.getProvider();
      } catch {
        provider = new anchor.AnchorProvider(connection, wallet, {});
        anchor.setProvider(provider);
  
        const programInstance = new anchor.Program(idl, PROGRAM_ID);
        setProgram(programInstance);
      }
    }, [connection, wallet]);
  
    const onClick = async () => {
      const newAccount = anchor.web3.Keypair.generate();
  
      const sig = await program.methods
        .initialize()
        .accounts({
          counter: newAccount.publicKey,
          user: wallet.publicKey,
          systemAccount: anchor.web3.SystemProgram.programId,
        })
        .signers([newAccount])
        .rpc();
  
      setTransactionUrl(`https://explorer.solana.com/tx/${sig}?cluster=devnet`);
      setCounter(newAccount.publicKey);
    };
  
    return <Button onClick={onClick}>Initialize Counter</Button>;
  };
  