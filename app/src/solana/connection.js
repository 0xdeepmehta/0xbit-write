import {
    clusterApiUrl,
    Connection
} from "@solana/web3.js";
import Wallet from "@project-serum/sol-wallet-adapter";

const connection = new Connection(clusterApiUrl('devnet'));
export const getConnection = () => connection;

const PROVIDER_URL = "https://www.sollet.io";
let wallet = new Wallet(PROVIDER_URL);
wallet.on('connect', publicKey => console.log('Connected to ' + publicKey.toBase58()));
wallet.on('disconnect', () => console.log('Disconnected'));
// await wallet.connect();


export const sendTxUsingExternalSignature = async (
    instructions,
    connection,
    feePayer,
    signersExceptWallet,
    wallet
) => {
    let tx = new Transaction().add(...instructions);
    tx.setSigners(
        ...(feePayer
            ? [(feePayer).publicKey, wallet.publicKey]
            : [wallet.publicKey]),
        ...signersExceptWallet.map(s => s.publicKey)
    );
    tx.recentBlockhash = (await connection.getRecentBlockhash("max")).blockhash;
    signersExceptWallet.forEach(acc => {
        tx.partialSign(acc);
    });
    let signed = await wallet.signTransaction(tx);
    let txid = await connection.sendRawTransaction(signed.serialize(), {
        skipPreflight: false,
        preflightCommitment: COMMITMENT
    });
    return connection.confirmTransaction(txid, COMMITMENT);
};

const connectToWallet = () => {
    if (!wallet.connected) {
        return wallet.connect();
    } else {
        return Promise.resolve();
    }
};

export const useWallet = async () => {
    await connectToWallet();
    return wallet;
};