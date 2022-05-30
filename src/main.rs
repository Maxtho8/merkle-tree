// Merkle Tree algorithm

use tx_module::tx::Tx;
use hash_module::hash::Hash::{hash_to_2hex};

fn get_root_hash(txs : Vec<Tx>) -> Vec<String>{
    let mut hash_root : Vec<String> = vec![];
    let mut txs_hashes : Vec<String> = vec![];
    for tx in txs{
        txs_hashes.push(hash_to_2hex(&tx.to_string()));
    }
    let mut txs_hashes_clone = txs_hashes.clone();
    while txs_hashes_clone.len() > 1 {
        let mut new_txs_hashes : Vec<String> = vec![];
        for i in 0..txs_hashes_clone.len()-1{
            let mut digest = txs_hashes_clone[i].clone();
            digest.push_str(&txs_hashes_clone[i+1]);
            new_txs_hashes.push(hash_to_2hex(&digest));
        }
        txs_hashes_clone = new_txs_hashes;
    }
    hash_root.push(txs_hashes_clone[0].clone());
    hash_root
}

    
fn main() {
    let  txs: Vec<Tx> = vec![Tx::new(1, "Bob".to_owned(), "Alice".to_owned()), 
                            Tx::new(1, "Alice".to_owned(), "Bob".to_owned()),
                           ];
    let hash_root = get_root_hash(txs);
    println!("{:?}", hash_root);
    
}
