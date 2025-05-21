import {
  AnchorProvider,
  Program,
  Provider,
  setProvider,
  Wallet,
} from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";

import { Veralux } from "../idl/veralux";

import IDL from "../idl/veralux.json";
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

export const GLOBAL_SEED = "global-authority";
export const PRESALE_SEED = "presale-authority";
export const TREASURY_SEED = "treasury-authority";
export const MULTISIG_SEED = "multisig-authority";

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
    public treasuryWallet: Wallet,
    public userWallet: Wallet,
    public owner1: Wallet,
    public owner2: Wallet,
    public owner3: Wallet,
    public owner4: Wallet,
    public owner5: Wallet,
    public tokenMint: PublicKey,
    public usdtMint: PublicKey
  ) {
    MockFactory.mockFactory = this;
  }

  static async create(): Promise<MockFactory> {
    const backendKeypair = Keypair.fromSecretKey(
      Uint8Array.from(JSON.parse(process.env.BACKEND_WALLET || "[]"))
    );

    const backendWallet = new Wallet(backendKeypair);

    const treasuryWallet = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.TREASURY_WALLET || "[]"))
      )
    );

    const userWallet = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.USER_WALLET || "[]"))
      )
    );

    const owner1 = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.OWNER1_WALLET || "[]"))
      )
    );

    const owner2 = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.OWNER2_WALLET || "[]"))
      )
    );

    const owner3 = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.OWNER3_WALLET || "[]"))
      )
    );

    const owner4 = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.OWNER4_WALLET || "[]"))
      )
    );

    const owner5 = new Wallet(
      Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.OWNER5_WALLET || "[]"))
      )
    );

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
        "Veralux-X",
        "https://arweave.net/1234567890",
        "VXx",
        connection,
        9,
        backendKeypair,
        treasuryWallet,
        program,
        1000000000
      );
    }

    let usdtMint: PublicKey;
    const existingUsdtMint = process.env.USDT_TOKEN;
    if (existingUsdtMint) {
      usdtMint = new PublicKey(existingUsdtMint);
    } else {
      const userKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env.USER_WALLET || "[]"))
      );

      usdtMint = await deployUsdtToken(
        "TEST_USDT",
        "https://arweave.net/1234567890",
        "T_USDT",
        connection,
        6,
        backendKeypair,
        userKeypair,
        1000000
      );
    }

    return new MockFactory(
      program,
      provider,
      connection,
      backendWallet,
      treasuryWallet,
      userWallet,
      owner1,
      owner2,
      owner3,
      owner4,
      owner5,
      tokenMint,
      usdtMint
    );
  }
}

const deployUsdtToken = (
  name: string,
  uri: string,
  symbol: string,
  connection: Connection,
  decimals: number = 9,
  backendKeypair: Keypair,
  userKeypair: Keypair,
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
          tokenOwner: fromWeb3JsPublicKey(userKeypair.publicKey),
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

const deployToken = async (
  name: string,
  uri: string,
  symbol: string,
  connection: Connection,
  decimals: number = 9,
  backendKeypair: Keypair,
  treasuryWallet: Wallet,
  program: Program<Veralux>,
  initialSupply?: number
) => {
  const umi = createUmi(connection).use(mplToolbox());
  umi.use(keypairIdentity(fromWeb3JsKeypair(backendKeypair)));

  const mint = generateSigner(umi);

  const [treasuryAuthorityPda] = PublicKey.findProgramAddressSync(
    [Buffer.from(TREASURY_SEED), treasuryWallet.publicKey.toBuffer()],
    program.programId
  );

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

  if (initialSupply && initialSupply > 0) {
    const mintAmount = BigInt(initialSupply * 10 ** decimals);

    builder = builder.add(
      mintV1(umi, {
        mint: mint.publicKey,
        tokenStandard: TokenStandard.Fungible,
        tokenOwner: fromWeb3JsPublicKey(treasuryAuthorityPda),
        amount: mintAmount,
      })
    );
  }

  builder.sendAndConfirm(umi, { confirm: { commitment: "confirmed" } });

  return Promise.resolve(toWeb3JsPublicKey(mint.publicKey));
};
