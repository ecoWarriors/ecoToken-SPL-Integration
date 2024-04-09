import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EcoMeme } from "../target/types/eco_meme";

describe("ecoMeme", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.EcoMeme as Program<EcoMeme>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
