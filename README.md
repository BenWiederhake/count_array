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
for myslice in count_array::over(myelements, mylength) {
    println!("Found: {}", myslice);
}
println!("Done!");
```

This should be really straight-forward.

## TODOs

- Everything
- Extend it to other integers
- Compare speed against `permutate`
- `StreamingIterator` implementation

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/mlmlpp/issues/new) or submit PRs.
