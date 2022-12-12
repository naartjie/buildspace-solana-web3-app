const anchor = require("@project-serum/anchor");
const { assert } = require("chai");
const { SystemProgram } = anchor.web3;

const test1 = async () => {
  console.log("🧨 Starting test YOLO 🧨");

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Myepicproject;
  const baseAccount = anchor.web3.Keypair.generate();
  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log(`🌭 your tx signature ${tx}`);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(`👁👁 gif count ${account.totalGifs}`);
  console.log(`typeof account.totalGifs -> ${typeof account.totalGifs}`);
  // assert(account.totalGifs === 0);

  await program.rpc.addGif("gif link goes here", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(`👁👁 gif count ${account.totalGifs}`);
  // assert(account.totalGifs === 1);
  console.log(`👁👁 gif list`, account.gifList);

  await program.rpc.upvote(0, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(
    `👁👁 any votes?`,
    account.gifList.map((gl) => gl.votes)
  );
};

const main = async () => {
  try {
    await test1();
    process.exit(0);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

main();
