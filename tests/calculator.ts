import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { assert } from "chai";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const newAccount = anchor.web3.Keypair.generate();

  const program = anchor.workspace.calculator as Program<Calculator>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .init(10)
    .accounts({
      signer: anchor.getProvider().wallet.publicKey,
      account: newAccount.publicKey
    })
    .signers([newAccount])
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert(account.num == 10)
  });
});
