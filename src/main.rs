//! Testing library usage of what it would look like to use this structure.
use playfair_rs::{Playfair, Cipher};

/// Main function
fn main() {
    // Create a new playfair structure, inputting the keyword you want to encrypt on
    let pf = Playfair::new("taxborn");

    // Encrypt a given string reference
    let res = pf.encrypt("Thanks for checking out my code!");
    println!("{res}");

    // Decrypt a given string reference
    let dec = pf.decrypt(&res);
    println!("{dec}"); 
}
