import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { BitWrite } from '../target/types/bit_write';

describe('bit_write', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.BitWrite as Program<BitWrite>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
