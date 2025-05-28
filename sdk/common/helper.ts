import {
  AnchorProvider,
  Program,
  Provider,
  setProvider,
  Wallet,
} from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";
import { generateSigner, keypairIdentity } from "@metaplex-foundation/umi";
import {
  fromWeb3JsKeypair,
  fromWeb3JsPublicKey,
  toWeb3JsPublicKey,
} from "@metaplex-foundation/umi-web3js-adapters";
import {
  createFungible,
  mintV1,
  TokenStandard,
} from "@metaplex-foundation/mpl-token-metadata";
import { resolve } from "path";
import { readFileSync } from "fs";

import { Veralux } from "../idl/veralux";

import IDL from "../idl/veralux.json";

export const GLOBAL_SEED = "global-authority";
export const PRESALE_SEED = "presale-authority";
export const TREASURY_SEED = "treasury-authority";
export const MULTISIG_SEED = "multisig-authority";
export const PENDING_MULTISIG_SEED = "pending-multisig-authority";
export const PROPOSAL_SEED = "proposal-authority";

export type SolanaProvider = {
  connection: Connection;
  wallet: Wallet;
};

const getTestConnection = (): Connection => {
  const url =
    "https://devnet.helius-rpc.com/?api-key=c468ac4b-f75f-422d-b7c2-b965484d3eaf";
  return new Connection(url, "confirmed");
};

export class MockFactory {
  static mockFactory: MockFactory;

  constructor(
    public program: Program<Veralux>,
    public provider: Provider,
    public connection: Connection,
    public backendWallet: Wallet,
    public owner1: Wallet,
    public owner2: Wallet,
    public owner3: Wallet,
    public owner4: Wallet,
    public treasuryWallet: Wallet,
    public LPWallet: Wallet,
    public teamWallet: Wallet,
    public charityWallet: Wallet,
    public user1: Wallet,
    public user2: Wallet,
    public user3: Wallet,
    public user4: Wallet,
    public user5: Wallet,
    public user6: Wallet,
    public user7: Wallet,
    public user8: Wallet,
    public user9: Wallet,
    public user10: Wallet,
    public user11: Wallet,
    public user12: Wallet,
    public user13: Wallet,
    public user14: Wallet,
    public user15: Wallet,
    public tokenMint: PublicKey,
    public usdcMint: PublicKey
  ) {
    MockFactory.mockFactory = this;
  }

  static async create(): Promise<MockFactory> {
    const backendKeypair = loadKeypair("./keypairs/main.json");
    const backendWallet = new Wallet(backendKeypair);

    const owner1Keypair = loadKeypair("./keypairs/owner1.json");
    const owner1 = new Wallet(owner1Keypair);

    const owner2Keypair = loadKeypair("./keypairs/owner2.json");
    const owner2 = new Wallet(owner2Keypair);

    const owner3Keypair = loadKeypair("./keypairs/owner3.json");
    const owner3 = new Wallet(owner3Keypair);

    const owner4Keypair = loadKeypair("./keypairs/owner4.json");
    const owner4 = new Wallet(owner4Keypair);

    const user1Keypair = loadKeypair("./keypairs/user1.json");
    const user1 = new Wallet(user1Keypair);

    const user2Keypair = loadKeypair("./keypairs/user2.json");
    const user2 = new Wallet(user2Keypair);

    const user3Keypair = loadKeypair("./keypairs/user3.json");
    const user3 = new Wallet(user3Keypair);

    const user4Keypair = loadKeypair("./keypairs/user4.json");
    const user4 = new Wallet(user4Keypair);

    const user5Keypair = loadKeypair("./keypairs/user5.json");
    const user5 = new Wallet(user5Keypair);

    const user6Keypair = loadKeypair("./keypairs/user6.json");
    const user6 = new Wallet(user6Keypair);

    const user7Keypair = loadKeypair("./keypairs/user7.json");
    const user7 = new Wallet(user7Keypair);

    const user8Keypair = loadKeypair("./keypairs/user8.json");
    const user8 = new Wallet(user8Keypair);

    const user9Keypair = loadKeypair("./keypairs/user9.json");
    const user9 = new Wallet(user9Keypair);

    const user10Keypair = loadKeypair("./keypairs/user10.json");
    const user10 = new Wallet(user10Keypair);

    const user11Keypair = loadKeypair("./keypairs/user11.json");
    const user11 = new Wallet(user11Keypair);

    const user12Keypair = loadKeypair("./keypairs/user12.json");
    const user12 = new Wallet(user12Keypair);

    const user13Keypair = loadKeypair("./keypairs/user13.json");
    const user13 = new Wallet(user13Keypair);

    const user14Keypair = loadKeypair("./keypairs/user14.json");
    const user14 = new Wallet(user14Keypair);

    const user15Keypair = loadKeypair("./keypairs/user15.json");
    const user15 = new Wallet(user15Keypair);

    const teamWalletKeypair = loadKeypair("./keypairs/team.json");
    const teamWallet = new Wallet(teamWalletKeypair);

    const treasuryWalletKeypair = loadKeypair("./keypairs/treasury.json");
    const treasuryWallet = new Wallet(treasuryWalletKeypair);

    const LPWalletKeypair = loadKeypair("./keypairs/LP.json");
    const LPWallet = new Wallet(LPWalletKeypair);

    const charityWalletKeypair = loadKeypair("./keypairs/charity.json");
    const charityWallet = new Wallet(charityWalletKeypair);

    const connection = await getTestConnection();

    const provider = new AnchorProvider(connection, backendWallet, {
      commitment: "confirmed",
    });
    setProvider(provider);

    const veraluxInterface = JSON.parse(JSON.stringify(IDL));
    const program = new Program<Veralux>(veraluxInterface) as Program<Veralux>;

    let tokenMint: PublicKey;
    const existingMint = process.env.VERALUX_TOKEN;
    if (existingMint) {
      tokenMint = new PublicKey(existingMint);
    } else {
      tokenMint = await deployToken(
        "VERALUX TOKEN",
        "https://gateway.pinata.cloud/ipfs/bafkreig2445aeloe56yw3z77qvjnndhwuqsjoxrdbngssebvldq4sb5nja",
        "VERALUXT",
        connection,
        9,
        backendKeypair,
        1000000000
      );
    }

    let usdcMint: PublicKey;
    const existingUsdtMint = process.env.USDC_TOKEN;
    if (existingUsdtMint) {
      usdcMint = new PublicKey(existingUsdtMint);
    } else {
      usdcMint = await deployToken(
        "USDC Test Token",
        "https://gateway.pinata.cloud/ipfs/bafkreifxmejoapgvn44ds6l2zmnltflw34mdm4ss4wp5nwruqi7ytthija",
        "USDC-TEST",
        connection,
        6,
        backendKeypair,
        1000000000
      );
    }

    return new MockFactory(
      program,
      provider,
      connection,
      backendWallet,
      owner1,
      owner2,
      owner3,
      owner4,
      treasuryWallet,
      LPWallet,
      teamWallet,
      charityWallet,
      user1,
      user2,
      user3,
      user4,
      user5,
      user6,
      user7,
      user8,
      user9,
      user10,
      user11,
      user12,
      user13,
      user14,
      user15,
      tokenMint,
      usdcMint
    );
  }
}

const deployToken = (
  name: string,
  uri: string,
  symbol: string,
  connection: Connection,
  decimals: number = 6,
  backendKeypair: Keypair,
  initialSupply?: number
): Promise<PublicKey> => {
  try {
    const umi = createUmi(connection).use(mplToolbox());
    umi.use(keypairIdentity(fromWeb3JsKeypair(backendKeypair)));

    const mint = generateSigner(umi);

    let builder = createFungible(umi, {
      name,
      uri,
      symbol,
      sellerFeeBasisPoints: {
        basisPoints: BigInt(0),
        identifier: "%",
        decimals: 2,
      },
      decimals,
      mint,
    });

    if (initialSupply) {
      builder = builder.add(
        mintV1(umi, {
          mint: mint.publicKey,
          tokenStandard: TokenStandard.Fungible,
          tokenOwner: fromWeb3JsPublicKey(backendKeypair.publicKey),
          amount: initialSupply * Math.pow(10, decimals),
        })
      );
    }

    builder.sendAndConfirm(umi, { confirm: { commitment: "confirmed" } });

    return Promise.resolve(toWeb3JsPublicKey(mint.publicKey));
  } catch (error) {
    console.error(error);
    return Promise.reject(error);
  }
};

export const loadKeypair = (filePath: string): Keypair => {
  const absolutePath = resolve(filePath);
  const keypairString = readFileSync(absolutePath, "utf-8");
  const keypairBuffer = Buffer.from(JSON.parse(keypairString));
  return Keypair.fromSecretKey(keypairBuffer);
};
