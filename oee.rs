extern crate rand;
use rand::Rng;
use std::env;

const CYPHERBYTES: usize = 32;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} file-to-encrypt", args[0]);
        return
    }

    let mut csprng = match rand::OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Could not find a cryptographically-secure PRNG on the system. Details: {}", e)
    };

    let mut cyphertext = [0u8; CYPHERBYTES];
    csprng.fill_bytes(&mut cyphertext);
    for x in &cyphertext { print!("{:02x}", *x); }
    println!("")
}
