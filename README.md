# playfair-rs - [Playfair Cipher](https://en.wikipedia.org/wiki/Playfair_cipher) in [Rust](https://www.rust-lang.org/).
Originally was an assignment for my CS 303: Databases and Information Security class, 
I wanted to take some more time and implement it again. This is my solution to implementing
the [Playfair Cipher](https://en.wikipedia.org/wiki/Playfair_cipher) in Rust.

## Design choices
Some implementations omit the letter 'q' in encryption/decryption, some omit 'j', and 
some equate 'i' to 'j'. For my implementation, I went with 'i' = 'j', since that is 
what the Wikipedia article's example followed, and a [random online playfair cipher](https://www.boxentriq.com/code-breaking/playfair-cipher)
website used by default. This allowed for easy verification of my implementation.

I also used a task runner (originally a Makefile, now a [justfile](justfile)) to enforce
strict commenting of all my functions to ensure I knew what was going on at that line and
function. As well thanks to the awesome [Rust ecosystem](https://www.rust-lang.org/tools),
it enables generation of documentation from a single `just doc` command. That documentation
can be found in the `./target/doc/playfair/` directory afterwards.

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
[`just ci`](justfile) runs every check CI enforces, in the same order, so a green run
locally means a green run on the PR:

```sh
just ci
```

`just --list` shows the individual recipes — `fmt`, `lint`, `build`, `test`, `doc` and
`bench` — if you want to run one at a time. Benchmarks are deliberately left out of `ci`,
being far too slow and noisy to gate a pull request on.

The workflow in [`.forgejo/workflows/ci.yml`](.forgejo/workflows/ci.yml) repeats these
commands rather than calling `just`, because `just` isn't packaged for Debian and
installing it into the build container would cost more than the checks themselves. The two
therefore have to be kept in sync by hand.

> **On NixOS** there is no `cc` on `PATH` by default, so any `cargo` command that links
> fails with ``linker `cc` not found``. `rustfmt` and `clippy` are separate packages too,
> so run recipes inside a shell that supplies all three:
>
> ```sh
> nix-shell -p gcc rustfmt clippy --run "just ci"
> ```

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
