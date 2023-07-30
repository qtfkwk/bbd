Library for encoding and decoding data to/from binary representations using the
[Braille Patterns Unicode Block characters](https://en.wikipedia.org/wiki/Braille_Patterns)

See the [documentation](https://docs.rs/bbd-lib) or the
[source](https://github.com/qtfkwk/bbd/blob/main/lib/src/lib.rs) for each
function for doctest examples.

See also the [bbd](https://crates.io/crates/bbd) crate which provides a CLI
utility.

```
use bbd_lib::*;

assert_eq!(encode(b"Hello\n", encode_nlbb, 0, 0), "⢄⠮⢦⢦⢾⢐");
assert_eq!(decode("⢄⠮⢦⢦⢾⢐", decode_nlbb), b"Hello\n");
```

