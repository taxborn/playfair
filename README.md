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

## Development
Every check CI enforces can be run locally:

```sh
cargo fmt --check
cargo clippy --all-targets -- -F clippy::missing_docs_in_private_items -D warnings
cargo build --all-targets
cargo test
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
```

`make` covers the docs, lint, test and benchmark steps in one go.

> **On NixOS** there is no `cc` on `PATH` by default, so any `cargo` command that links
> fails with ``linker `cc` not found``. Prefix with `nix-shell -p gcc --run "..."`, or add
> `gcc` to the dev shell. `cargo fmt` and `cargo clippy` additionally need `rustfmt` and
> `clippy` in that shell, since nixpkgs ships them as separate packages.

## Releasing
[`Cargo.toml`](Cargo.toml) is the single source of truth for the version, and the git tag
is derived from it — never the other way round. There is no manual tagging or publishing
step.

1. In your PR, bump `version` in [`Cargo.toml`](Cargo.toml) following semver.
2. Get the PR green and merge it into `main`.
3. [`.forgejo/workflows/release.yml`](.forgejo/workflows/release.yml) does the rest: it
   reads the version, checks whether `v<version>` is already tagged, and if not, runs the
   tests, verifies the package, publishes to crates.io, then pushes the tag.

A merge that does not bump the version is simply not a release — the workflow finds the
existing tag and exits. That makes it safe to merge as often as you like, and safe to
re-run a failed release by merging again.

Publishing happens *before* tagging on purpose: crates.io is immutable and a tag is not,
so a tag always means "this version really is on crates.io". In the one awkward case where
the publish succeeds but the tag push fails, tag it by hand:

```sh
git tag -a v<version> -m "Release v<version>"
git push origin v<version>
```

Two things the release depends on:

- A `CARGO_REGISTRY_TOKEN` repository secret holding a crates.io API token with publish
  rights on the `playfair` crate. The tag push uses the automatic `GITHUB_TOKEN`, which
  Forgejo grants repository write access on `push` events, so it needs no setup.
- `rust-version` in [`Cargo.toml`](Cargo.toml) staying honest. Bump it whenever a change
  starts leaning on a newer stdlib API, so downstream users get a clear toolchain error
  rather than a confusing compile failure.
