CLI utility for encoding and decoding data to/from binary representations using the
[Braille Patterns Unicode Block characters](https://en.wikipedia.org/wiki/Braille_Patterns)

See also the [bbd-lib](https://crates.io/crates/bbd-lib) library crate.

# Usage

```text
$ bbd -h
Binary Braille Dump

Encode/decode data to/from Braille Patterns Unicode Block characters

Usage: bbd [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Input file(s); [default: "-" (stdin)]

Options:
  -d              Decode Braille characters to bytes using the given style;
                  ignores wrapping
  -s <STYLE>      Style (1) [default: nlbb] [possible values: bcd, direct, nlbb,
                  nlbt, nrbb, nrbt]
  -c <N>          Wrap to N columns ("bytes") per line; 0: disable wrapping
                  [default: 64]
  -m              Markdown output
  -h, --help      Print help
  -V, --version   Print version

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

```text
$ bbd -V
bbd 0.4.1
```

# Examples

```text
$ echo Hello |bbd
⢄⠮⢦⢦⢾⢐
```

```text
$ echo "⢄⠮⢦⢦⢾⢐" |bbd -d
Hello
```

