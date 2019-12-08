use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, Uniform, Standard};
use std::io;

fn main() {
    println!("🔒 Ready to generate a password via CLI like a real haX0r? 🔒");
    println!("Input your password length");

    let mut password_length = String::new();

    io::stdin().read_line(&mut password_length)
        .expect("Failed to read line");

    let len_str = password_length.trim();

    println!("length = {:?}", len_str);
    let p_length = len_str.parse().unwrap_or(14);

    let rng = thread_rng();

    let s: String = rng.sample_iter(Alphanumeric).take(p_length).collect();
    println!("Your super l33t password is: {:?}", s);
}
