[Library](https://crates.io/crates/bbd-lib) and
[CLI utility](https://crates.io/crates/bbd) for encoding and decoding data
to/from binary representations using the
[Braille Patterns Unicode Block characters](https://en.wikipedia.org/wiki/Braille_Patterns)

See the [documentation](https://docs.rs/bbd-lib) or the
[source](https://github.com/qtfkwk/bbd/blob/main/lib/src/lib.rs) for each
function for doctest examples.

# Library

```
use bbd_lib::*;

assert_eq!(encode(b"Hello\n", encode_nlbb, 0, 0), "⢄⠮⢦⢦⢾⢐");
assert_eq!(decode("⢄⠮⢦⢦⢾⢐", decode_nlbb), b"Hello\n");
```

# CLI

## Usage

```text
$ bbd -h
!run:../target/release/bbd -h
```

## Examples

```text
$ echo Hello |bbd
!run:echo Hello |../target/release/bbd
```

```text
$ echo "⢄⠮⢦⢦⢾⢐" |bbd -d
!run:echo "⢄⠮⢦⢦⢾⢐" |../target/release/bbd -d
```

