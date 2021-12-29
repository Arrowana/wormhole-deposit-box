import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { WormholeDepositBox } from '../target/types/wormhole_deposit_box';

describe('wormhole-deposit-box', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.WormholeDepositBox as Program<WormholeDepositBox>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
