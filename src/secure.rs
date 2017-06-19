use ring::aead::{seal_in_place, open_in_place, AES_256_GCM, SealingKey};
use ring::rand::{SystemRandom, SecureRandom};
use clap::ArgMatches;

pub fn secure(matches: &ArgMatches) {
    println!("Inside secure");
    let random_nonce = SystemRandom::new();
    let mut nonce: [u8; 92] = [0; 92];
    let res = random_nonce.fill(&mut nonce);
    // println!("{:?}", nonce);
    for i in nonce.into_iter() {
        print!("{}", i);
    }
    print!("\n");
    println!("That's a long key");
    let key_bytes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                     22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    let key = SealingKey::new(&AES_256_GCM, &key_bytes).unwrap();
    // match key {
    //     Ok(key) => key,
    //     // Ok(key) => println!("{:?}", key),
    //     Err(err) => println!("Error: {:?}", err),
    // }
    let mut string: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];

    let res = seal_in_place(&key, &nonce, &[], &mut string, 0);
    println!("{:?}", res);

    println!("{:?}", string);
}
