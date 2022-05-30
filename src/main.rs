// Merkle Tree algorithm

use std::str::from_utf8;

use sha2::{Sha256, Digest};


struct Tx{
    amount : u64,
    sender : String,
    receiver : String,
}

impl Tx{
    fn new(amount : u64, sender : String, receiver : String) -> Tx{
        Tx{
            amount : amount,
            sender : sender,
            receiver : receiver,
        }
    }
    fn to_string(&self) -> String{
        format!("{}:{}:{}", self.amount, self.sender, self.receiver)
    }
} 

fn hash_function(digest : &str) -> String{
        //hash a string sha256
        let mut hash: String = "".to_owned();
        let mut hasher = Sha256::new();
        hasher.update(digest);
        let result = hasher.finalize(); 
        // for each byte in result 
        for byte in result.iter() {
            hash.push_str(&format!("{:02x}", byte)[..]);  
        }   
        return hash;
}

// markle tree vec of hashes
fn merkle_tree(txs : Vec<Tx>) -> Vec<String>{
    let mut tree : Vec<String> = vec![];
    let mut txs_hashes : Vec<String> = vec![];
    for tx in txs{
        txs_hashes.push(hash_function(&tx.to_string()));
    }
    let mut txs_hashes_clone = txs_hashes.clone();
    while txs_hashes_clone.len() > 1 {
        let mut new_txs_hashes : Vec<String> = vec![];
        for i in 0..txs_hashes_clone.len()-1{
            let mut digest = txs_hashes_clone[i].clone();
            digest.push_str(&txs_hashes_clone[i+1]);
            new_txs_hashes.push(hash_function(&digest));
        }
        txs_hashes_clone = new_txs_hashes;
    }
    tree.push(txs_hashes_clone[0].clone());
    tree
}

    
fn main() {

    let  txs: Vec<Tx> = vec![Tx::new(1, "Bob".to_owned(), "Alice".to_owned()), 
                            Tx::new(1, "Alice".to_owned(), "Bob".to_owned()),
                           ];
    let tree = merkle_tree(txs);
    println!("{:?}", tree);
    
}
