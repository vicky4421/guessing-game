import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GuessingGame } from "../target/types/guessing_game";
import { networkStateAccountAddress, Orao, randomnessAccountAddress } from "@orao-network/solana-vrf";

describe("guessing-game", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.GuessingGame as Program<GuessingGame>;

  let force_seed = anchor.web3.Keypair.generate().publicKey.toBuffer();

  const vrf = new Orao(provider);
  const random = randomnessAccountAddress(force_seed);
  const treasury = new anchor.web3.PublicKey("7Wa5jXzh5SWPL8AnDM3fkPpSc391nys7zWzyNnabGXH");

  it("Got a random number from Orao VRF program", async () => {
    console.log("is this working?");
    const tx = await program.methods
      .init([...force_seed])
      .accounts({
        payer: provider.wallet.publicKey,
        treasury,
        oraVrf: vrf.programId,
        random,
        networkState: networkStateAccountAddress(),
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc({ skipPreflight: true });

    await vrf.waitFulfilled(force_seed);
  });
});
