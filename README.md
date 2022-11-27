# playfair-rs - [Playfair Cipher](https://en.wikipedia.org/wiki/Playfair_cipher) in [Rust](https://www.rust-lang.org/).
Originally was an assignment for my CS 303: Databases and Information Security class, 
I wanted to take some more time and implement it again. This is my solution to implementing
the [Playfair Cipher](https://en.wikipedia.org/wiki/Playfair_cipher) in Rust.

## Design choices
Some implementations omit the letter 'q' in encryption/decryption, some omit 'j', and 
some equate 'i' to 'j'. For my implementation, I went with 'i' = 'j', since that is 
what the Wikipedia article's example followed, and a [random online playfair cipher](https://www.boxentriq.com/code-breaking/playfair-cipher)
website used by default. This allowed for easy verification of my implementation.

I also used a [Makefile](Makefile) to enforce strict commenting of all my functions 
to ensure I knew what was going on at that line and function. As well thanks to the awesome
[Rust ecosystem](https://www.rust-lang.org/tools), it enables generation of documentation
with just a single `make` command. That documentation can be found in the `./target/doc/playfair_rs/`
directory, following execution of the `make` command.

I also used testing extensively, for each part from keyword generation, matrix computation, 
character location, to [full integration testing](./tests/playfair_tests.rs). Combining this
with assertions in the code, I have a pretty good idea that my code is correct.

## Examples
You can see an example in [main.rs](./src/main.rs), or here is a simple shown implementation:

**Encryption steps:**
```rust
use playfair::{Cipher, Playfair};

// Example from https://en.wikipedia.org/wiki/Playfair_cipher.
fn main() {
    let pf = Playfair::new("playfair example");
    let out = pf.encrypt("Hide the gold in the tree stump.");

    // out = bmodzbxdnabekudmuixmmouvif
}
```

**Decryption steps:**
```rust
use playfair::{Cipher, Playfair};

// Example from https://en.wikipedia.org/wiki/Playfair_cipher.
fn main() {
    let pf = Playfair::new("playfair example");
    let out = pf.decrypt("bmodzbxdnabekudmuixmmouvif");

    // out = hidethegoldinthetrexestump
    // NOTE: the extra 'x' here ^ is expected since it was inserted during the encryption process.
    // Read more about the Playfair cipher to understand why.
}
```
