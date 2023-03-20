use bitcoin::{blockdata::transaction::OutPoint, util::address::Address, Network, PrivateKey, PublicKey, Script, Transaction, TxIn, TxOut};
use secp256k1::{Secp256k1, Message, All, Signature};

fn main() {
    // Set up the transaction inputs and outputs
    let txin = TxIn {
        previous_output: OutPoint::new([0; 32], 0),
        script_sig: Script::default(),
        sequence: 0xFFFFFFFF,
        witness: vec![],
    };
    let txout = TxOut {
        value: 100_000,
        script_pubkey: Address::from_str("1someaddress").unwrap().script_pubkey(),
    };
    let tx = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![txin],
        output: vec![txout],
    };

    // Sign the transaction
    let secp = Secp256k1::new();
    let privkey = PrivateKey::from_wif("my_private_key").unwrap();
    let pubkey = PublicKey::from_private_key(&secp, &privkey);
    let msg = Message::from_slice(&tx.hash().as_ref()).unwrap();
    let sig = secp.sign(&msg, &privkey.key).serialize_der().to_vec();
    let pubkey_ser = pubkey.serialize().to_vec();
    let witness = vec![sig, pubkey_ser];
    tx.input[0].witness = witness;

    // Print the signed transaction hex
    let signed_tx_hex = tx.clone().consensus_encode().unwrap().to_hex();
    println!("Signed transaction hex: {}", signed_tx_hex);
}
