CLI utility for encoding and decoding data to/from binary representations using the
[Braille Patterns Unicode Block characters](https://en.wikipedia.org/wiki/Braille_Patterns)

See also the [bbd-lib](https://crates.io/crates/bbd-lib) library crate.

# Usage

```text
$ bbd -h
!run:../../target/release/bbd -h
```

```text
$ bbd -V
!run:../../target/release/bbd -V
```

# Examples

```text
$ echo Hello |bbd
!run:echo Hello |../../target/release/bbd
```

```text
$ echo "⢄⠮⢦⢦⢾⢐" |bbd -d
!run:echo "⢄⠮⢦⢦⢾⢐" |../../target/release/bbd -d
```

