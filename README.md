# count_array

> Count an array as if it was a number

Imagine you have an "array", and the first position (`myarray[0]`)
is considered to be the least-significant position.
Then one can count over that.

This library provides a Rust library and a very minimalistic interface to do this.
Currently, the implementation is restricted to `u32`.

Note that for permutations, you're better off with things
like [`permutate`](https://crates.io/crates/permutate)
or [`masked_permute`](https://github.com/BenWiederhake/masked_permute).

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Performance](#performance)
- [TODOs](#todos)
- [Contribute](#contribute)

## Background

Imagine you want to iterate over all possible compinations, e.g.:

```
[0, 0, 0, 0]
[0, 0, 0, 1]
[0, 0, 0, 2]
[0, 0, 0, 3]
[0, 0, 1, 0]
[0, 0, 1, 1]
[0, 0, 1, 2]
[0, 0, 1, 3]
[0, 0, 2, 0]
…
```

This package provides a simple, no-dependency way to do it.

## Install

### Rust

Add at an appropriate position to your `Cargo.toml`:

```TOML
[dependencies]
count_array = { git = "https://github.com/BenWiederhake/count_array.git" }
```

<!-- FIXME: Test this. -->

## Usage

Just use it!

```Rust
extern crate count_array;

let mylength = 2;
let myelements = 3;
let mut counter = count_array::over(myelements, mylength);
while {
    println!("Found: {:?}", counter.read());
    !counter.inc()
} { }
println!("Done!");
```

This is mostly straight-forward, except that the `Counter` is not usable as an iterator.

## Performance

It seems to take around 8 nanoseconds per step, so barely any overhead at all.
Locality should be mostly preserved, as all data will be stored in two contiguous regions:
`Vec` and `Counter` metadata; and the actual data referenced from `Vec`.

```
test bench_000027 ... bench:         281 ns/iter (+/- 21)
                                ==>   10.4 ns/step (+/- 0.8)
test bench_001024 ... bench:       8,619 ns/iter (+/- 286)
                                ==>    8.4 ns/step (+/- 0.3)
test bench_100000 ... bench:     823,473 ns/iter (+/- 47,699)
                                ==>    8.2 ns/step (+/- 0.5)
```

## TODOs

- Everything
- Extend it to other integers
- Compare speed against `permutate`
- `StreamingIterator` implementation

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/mlmlpp/issues/new) or submit PRs.
