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
Binary Braille Dump

Encode/decode data to/from Braille Patterns Unicode Block characters

Usage: bbd [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Input file(s); [default: "-" (stdin)]

Options:
  -d             Decode Braille characters to bytes using the given style;
                 ignores wrapping
  -s <STYLE>     Style (1) [default: nlbb] [possible values: bcd, direct, nlbb,
                 nlbt, nrbb, nrbt]
  -c <N>         Wrap to N columns ("bytes") per line; 0: disable wrapping
                 [default: 64]
  -m             Markdown output
  -h, --help     Print help
  -V, --version  Print version

---

Notes:

1. Styles:
    * `bcd`: Binary Coded Decimal of byte values 0-99
    * `direct`: Direct encoding using the standard Braille dot values
    * `nlbb`: Most significant nibble (MSN) left column, most significant bit
      (MSB) bottom row. This is the default style.
    * `nlbt`: MSN left column, MSB top row
    * `nrbb`: MSN right column, MSB bottom row
    * `nrbt`: MSN right column, MSB top row
```

## Examples

```text
$ echo Hello |bbd
⢄⠮⢦⢦⢾⢐
```

```text
$ echo "⢄⠮⢦⢦⢾⢐" |bbd -d
Hello
```

