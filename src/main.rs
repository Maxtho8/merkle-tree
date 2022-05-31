// Merkle Tree algorithm

use tx_module::tx::Tx;
use hash_module::hash::{hash::{hash_to_2hex}};



// get merkle root hash
fn get_merkle_root_hash(txs : Vec<Tx>) -> String {
    let mut txs_hashes : Vec<String> = vec![];
    for tx in txs{
        txs_hashes.push(hash_to_2hex(&tx.to_string()));
    }
    while txs_hashes.len() > 1 {
        let mut temp_hash : Vec<String> = vec![];
        if txs_hashes.len() % 2 != 0 {
            // push last element of 
            txs_hashes.push(txs_hashes[txs_hashes.len() - 1].clone());
        }
        for i in (0..txs_hashes.len()-1).step_by(2) {
            let mut digest : String =  txs_hashes[i].clone();
            digest.push_str(&txs_hashes[i+1]);
            temp_hash.push(hash_to_2hex(&digest));
        }
        println!("{:?}", temp_hash);
        txs_hashes = temp_hash;
    }
    txs_hashes[0].clone()
}

    
fn main() {
    let  txs: Vec<Tx> = vec![Tx::new(1, "Bob".to_owned(), "Alice".to_owned()), 
                            Tx::new(1, "Alice".to_owned(), "Bob".to_owned()),
                            Tx::new(1, "Alice".to_owned(), "Boby".to_owned()),
                            Tx::new(1, "Alice".to_owned(), "Boby".to_owned()),
                            Tx::new(1, "Jacques".to_owned(), "Boby".to_owned()),
                           ];
    let hash_root = get_merkle_root_hash(txs);
    println!("{:?}", hash_root);
    
}
