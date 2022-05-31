
pub mod hash {

    use sha2::{Sha256, Digest};

    pub fn hash_to_2hex(digest : &str) -> String{
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
}}