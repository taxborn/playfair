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
> fails with ``linker `cc` not found``, and `rustfmt`/`clippy` are separate packages from
> `rustc`. [`shell.nix`](shell.nix) supplies all of them plus `just`:
>
> ```sh
> nix-shell        # then: just ci
> ```

## Releasing
Releases are deliberately manual. CI checks every pull request but never publishes, so
that each release gets written notes and a GPG-signed tag rather than appearing silently
on merge. [`Cargo.toml`](Cargo.toml) remains the single source of truth for the version.

1. Bump `version` in [`Cargo.toml`](Cargo.toml) following semver. If the change leans on a
   newer stdlib API, bump `rust-version` in the same commit so downstream users get a clear
   toolchain error rather than a confusing compile failure.
2. Merge that through a PR as normal, so CI has checked it.
3. On `main`, with a clean tree, confirm everything still passes:

   ```sh
   nix-shell        # then: just ci
   ```

4. Verify the package exactly as crates.io will see it. This refuses to run on a dirty
   tree, which is a useful guard rather than an annoyance:

   ```sh
   cargo publish --dry-run
   ```

5. Publish:

   ```sh
   cargo publish
   ```

6. Tag it, signed, and push the tag:

   ```sh
   git tag -s v<version> -m "Release v<version>"
   git push origin v<version>
   ```

7. Write the release notes against that tag in Forgejo, under **Releases → New release**.

Publish before tagging: crates.io is immutable and a tag is not, so a tag always means
"this version really is on crates.io". A failed publish can simply be fixed and retried; a
tag that got ahead of a failed publish would be a lie you then have to clean up.

Publishing needs a crates.io API token with rights on the `playfair` crate — `cargo login`
stores it in `~/.cargo/credentials.toml`. Signing needs `user.signingkey` set and
`commit.gpgsign` on, which `git tag -s` picks up automatically.

> Tags before `v1.0.3` were pushed by CI and are annotated but **not** signed. Only tags
> from `v1.0.3` onward carry a signature.
