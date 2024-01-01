

## What is the purpose of this repository
- It is an example of how a benchmark can be organized.

## Any explanations?

- Well, it has 3 functions in the `src/lib.rs` file and corresponding benchmarks in the `benches/bench.rs` file.
- To execute this benchmark, we need to set up a `nightly` Rust toolchain and then run it with `cargo bench`.
- Personally, I just created a file `rust-toolchain` in the root of this repository, which contains only the name of the toolchain -- `nightly`.

## About this benchmark

- `bench_css_class_contains_any` -- wins in any case. It is much faster than others.


The worst scenario -- we didn't find any target class in the class string, but we wasted time.

```
test tests::bench_css_class_contains_any       ... bench:          64 ns/iter (+/- 10)
test tests::bench_css_class_contains_split     ... bench:         160 ns/iter (+/- 11)
test tests::bench_css_class_contains_substring ... bench:         131 ns/iter (+/- 3)
```

So the conclusion of this benchmark is that: `any` after a split is much effective than `contains`.