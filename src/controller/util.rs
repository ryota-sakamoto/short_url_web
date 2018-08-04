use crypto::{digest::Digest, sha2::Sha256};
use rand::{thread_rng, Rng};

pub fn generate_id(len: usize) -> String {
    let char_vec: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"
        .chars()
        .collect();
    let mut rng = thread_rng();
    let mut id = String::new();
    for _ in 0..len {
        let n = rng.gen_range(0, char_vec.len());
        id.push(char_vec[n]);
    }
    id
}

pub fn generate_password_hash(password: Option<String>, salt: &str) -> Option<String> {
    password.map(|p| sha256(&format!("{}{}", p, salt)))
}

pub fn sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s);
    hasher.result_str()
}
