use drand_verify::{derive_randomness, g1_from_fixed, verify};

use hex;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use sha2::{Digest, Sha256};

fn main() {
    let mut PK_LEO_MAINNET = [0u8; 48];
    hex::decode_to_slice("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31", &mut PK_LEO_MAINNET).unwrap();
    // TODO: no hardcoding
    let round_number = 1337;
    let api_baseurl = "https://drand.cloudflare.com";
    let chainHash = "8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce";
    let pubkey = g1_from_fixed(PK_LEO_MAINNET).unwrap();
    let signature = hex::decode("80d95247ddf1bb3acf5738497a5f10406be283144603f63d714bb1a44ff6b93285ae2697fffeb50c68862bd9fbecd4b204b1798d2686b4ac5d573615031d9d67e6168bde9a7adf1161430a498ca701a25c216aee3e38ffd5290369034fa050a2").unwrap();

    // TODO
    //dbg!(surf::get(format!("{api_baseurl}/public/{round_number}")));
    verify(&pubkey, round_number, &[], &signature).unwrap();
    let randomness = derive_randomness(&signature);
    println!("{}", hex::encode(randomness));

    // simulate shuf -n 3
    let mut input = vec!("Alice", "Bob", "Carla", "David");
    let input_len = input.len();
    let output_len = 3;
    for i in 0..output_len {
        let mut h = Sha256::new();
        h.update(randomness);
        h.update(BigUint::from(i).to_bytes_be());
        let r = BigUint::from_bytes_be(&h.finalize()[..]);
        input.swap(i, i+ (r % (input_len -1 - i)).to_usize().unwrap());
    }
    input.resize_with(output_len, || panic!("unreachable"));
    let output = input;
    println!("{:?}", output);
}
