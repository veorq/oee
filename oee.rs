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

    let mut rng = rand::thread_rng();

    let mut cyphertext = [0u8; CYPHERBYTES];
    rng.fill_bytes(&mut cyphertext);
    for x in &cyphertext { print!("{:02x}", *x); }
    println!()
}
