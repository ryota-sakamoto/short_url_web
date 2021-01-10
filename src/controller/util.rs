use crypto::{digest::Digest, sha2::Sha256};
use rand::{thread_rng, Rng};

pub fn generate_id(len: usize) -> String {
    let char_vec: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"
        .chars()
        .collect();
    let mut rng = thread_rng();
    let mut id = String::new();
    for _ in 0..len {
        let n = rng.gen_range(0..char_vec.len());
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

#[test]
fn generate_id_test() {
    const ID_LEN: usize = 8;

    let p1 = generate_id(ID_LEN);
    let p2 = generate_id(ID_LEN);

    assert!(p1 != p2);
}

#[test]
fn generate_password_hash_test() {
    let ph = generate_password_hash(None, "hash");
    assert_eq!(ph, None);

    let ph = generate_password_hash(Some("".to_string()), "\n");
    assert_eq!(
        ph,
        Some("01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b".to_string())
    );
}

#[test]
fn sha256_test() {
    assert_eq!(
        "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b",
        sha256("\n")
    );
    assert_eq!(
        "6b3a55e0261b0304143f805a24924d0c1c44524821305f31d9277843b8a10f4e",
        sha256("password\n")
    );
}
