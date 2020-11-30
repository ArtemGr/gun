# gun

The [Gun database](https://github.com/amark/gun), in Rust

# status and direction

## exploratory

While the version is “0.*” the port might align towards exploratory programming and rapid prototyping. [Performance Testing Speed Development](https://youtu.be/BEqH-oZ4UXI) is welcome, but at this stage it is okay to have performance gaps and regressions in the code

## nightly

I want to allow us to use `non_ascii_idents` and other such Nightly features. Presently tested on nightly-2020-11-20

## no_std

Out of the box we likely lack the `no_std` encryption facilities necessary to run this in Linux kernel and WASM, but we might use `c2rust` in the future to bridge that gap

## bounty-driven

I prefer to see myself [as a maintainer](https://github.com/subdavis/Tusk/issues/11#issuecomment-359661411), consolidating a community effort towards exploring Gun, porting it and then improving the port

To that end I will be playing with forward tasks/issues and listing them [on Reddit maybe](https://www.reddit.com/r/rust/comments/fm2cbq/bug_bounty_inconsistent_performance/), [Bountysource](https://www.reddit.com/r/rust/comments/k17f0k/this_week_in_rust_366/gdxrh6q/) and [This Week In Rust](https://users.rust-lang.org/t/twir-call-for-participation/4821)

I'd welcome guidance in that
