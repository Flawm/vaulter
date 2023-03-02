import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Vaulter } from '../target/types/vaulter';

describe('vaulter', async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vaulter as Program<Vaulter>,
    programId = program.programId,
    signer = program.provider.wallet.payer,
    payer = program.provider.wallet.publicKey,
    mint = new anchor.web3.PublicKey(
      '3sBGUL1rEL9eMJgeQsjFg2s94mnfbHujx5zuWMGBuwJJ',
    ),
    delegate = anchor.web3.Keypair.fromSeed(payer.toBytes()),
    token = new anchor.web3.PublicKey(
      '3PurtRaxs6znwsac2zdADCtU7BZRyidLWSZmkab2iKde',
    ),
    systemProgram = new anchor.web3.PublicKey(
      '11111111111111111111111111111111',
    ),
    metaProgram = new anchor.web3.PublicKey(
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
    ),
    lookup = (
      await anchor.web3.PublicKey.findProgramAddress(
        [delegate.publicKey.toBuffer()],
        programId,
      )
    )[0],
    meta = (
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from('metadata'), metaProgram.toBuffer(), mint.toBuffer()],
        metaProgram,
      )
    )[0],
    me = (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from('metadata'),
          metaProgram.toBuffer(),
          mint.toBuffer(),
          Buffer.from('edition'),
        ],
        metaProgram,
      )
    )[0],
    tokenProgram = anchor.utils.token.TOKEN_PROGRAM_ID;

  it('It delegates', async () => {
    const tx = new anchor.web3.Transaction();
    tx.add(
      await program.instruction.init({
        accounts: {
          payer,
          lookup,
          delegate: delegate.publicKey,
          systemProgram,
        },
      }),
    );
    tx.add(
      await program.instruction.giveAuthority({
        accounts: {
          payer,
          mint,
          lookup,
          token,
          delegate: delegate.publicKey,
          tokenProgram,
          systemProgram,
        },
      }),
    );
    let a = await program.provider.sendAndConfirm(tx, [signer], {
      skipPreflight: true,
      commitment: 'confirmed',
    });
    console.log(a);
  });

  it('It freezes', async () => {
    const tx = new anchor.web3.Transaction();
    tx.add(
      await program.instruction.freezeMpl({
        accounts: {
          payer: delegate.publicKey,
          mint,
          token,
          metaProgram,
          me,
          tokenProgram,
          systemProgram,
        },
      }),
    );

    let a = await program.provider.connection.sendTransaction(tx, [delegate], {
      skipPreflight: true,
    });

    console.log(a);
  });

  it('It thaws', async () => {
    //
    const tx = new anchor.web3.Transaction();
    tx.add(
      await program.instruction.thawMpl({
        accounts: {
          payer: delegate.publicKey,
          mint,
          token,
          metaProgram,
          me,
          tokenProgram,
          systemProgram,
        },
      }),
    );

    let a = await program.provider.connection.sendTransaction(tx, [delegate], {
      skipPreflight: true,
    });

    console.log(a);
  });

  it('It revokes', async () => {
    //
    let a = await program.methods
      .removeAuthority()
      .accounts({
        payer,
        mint,
        lookup,
        token,
        tokenProgram,
        delegate: delegate.publicKey,
        systemProgram,
      })
      .rpc();
    console.log(a);
  });
});
