import { CounterSolana } from "../target/types/counter_solana";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";


describe("solana-counter", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.CounterSolana as Program<CounterSolana>;

    const base_account = anchor.web3.Keypair.generate();

    it("Is initialized!", async () => {
        const tx = await program.methods
        .initialize()
        .accounts({baseAccount: base_account.publicKey})
        .signers([base_account])
        .rpc();

        const account = await program.account.baseAccount.fetch(base_account.publicKey);
        expect(account.count.toNumber() === 0);

    })
    it("Can increment!", async () => {
        const tx = await program.methods
        .update()
        .accounts({baseAccount: base_account.publicKey, user: provider.wallet.publicKey})
        .rpc();

        const account = await program.account.baseAccount.fetch(base_account.publicKey);
        expect(account.count.toNumber() === 1);
    })

});
