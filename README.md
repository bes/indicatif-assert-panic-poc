# indicatif debug assert panic poc

For `indicatif` version `0.17.7` this Proof-of-Concept crashes immediately when run in a
terminal on my computer, with this setup:

* MacBook Pro (M2 Max)
* iTerm 2 Build 3.4.22
* Terminal window is narrower than 113 columns (i.e. crashes on 112 but not 113).

Crashes with this message:

```shell
indicatif-assert-panic-poc% cargo run                                                      101 ↵
Compiling indicatif-assert-panic-poc v0.1.0 (.../repos/github/indicatif-assert-panic-poc)
Finished dev [unoptimized + debuginfo] target(s) in 0.30s
Running `target/debug/indicatif-assert-panic-poc`
⠁
thread 'main' panicked at .../.cargo/registry/src/index.crates.io-6f17d22bba15001f/indicatif-0.17.7/src/draw_target.rs:501:9:
assertion failed: self.orphan_lines_count <= self.lines.len()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'tokio-runtime-worker' panicked at .../.cargo/registry/src/index.crates.io-6f17d22bba15001f/indicatif-0.17.7/src/draw_target.rs:132:65:
called `Result::unwrap()` on an `Err` value: PoisonError { .. }
```
