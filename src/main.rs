const N: usize = 32;

fn create_random32() -> [u8;N] {
    let mut d: [u8;N] = [0;N];
    for i in 0..31 {
        d[i] = rand::random();
    }
    d
}

fn pubkey_from_privkey(privkey: &[u8;N]) -> [u8;32] {
    if let Ok(secret) = ed25519_dalek::SecretKey::from_bytes(privkey) {
        let public = ed25519_dalek::PublicKey::from(&secret);
        public.to_bytes()
    } else {
        [0;N]
    }
}

fn is_valid_pubkey(d: &[u8;N]) -> bool {
    if curve25519_dalek::edwards::CompressedEdwardsY::from_slice(&d.as_ref())
        .decompress()
        .is_some()
    {
        return false;
    }
    return true;
}

fn test_pubkeys1(n: u32) {
    let mut cnt_inv = 0;
    for _i in 0..n {
        let pubkey = create_random32();
        if !is_valid_pubkey(&pubkey) {
            cnt_inv += 1;
        }
    }
    println!("Checked {} random public keys;  Number of invalid public keys {}", n, cnt_inv);
}

fn test_from_privkeys1(n: u32) {
    let mut cnt_inv = 0;
    for _i in 0..n {
        let privkey = create_random32();
        let pubkey = pubkey_from_privkey(&privkey);
        if !is_valid_pubkey(&pubkey) {
            cnt_inv += 1;
        }
    }
    println!("Checked {} random private keys;  Number of invalid public keys {}", n, cnt_inv);
}

fn test_pubkeys(n1: u32, n2: u32) {
    for _i in 0..n1 {
        test_pubkeys1(n2);
        test_from_privkeys1(n2);
    }
}

fn main() {
    println!("Hello World!");
    create_random32();
    test_pubkeys(10, 10000);
}
