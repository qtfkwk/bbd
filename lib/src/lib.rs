#![doc = include_str!("../README.md")]

use lazy_static::lazy_static;

// Braille dot values given in LSB to MSB order for each "style"
const NLBB: &[u32; 8] = &[8, 16, 32, 128, 1, 2, 4, 64];
const NLBT: &[u32; 8] = &[128, 32, 16, 8, 64, 4, 2, 1];
const NRBB: &[u32; 8] = &[1, 2, 4, 64, 8, 16, 32, 128];
const NRBT: &[u32; 8] = &[64, 4, 2, 1, 128, 32, 16, 8];

// BCD
const TENS: &[u8] = &[0x00, 0x40, 0x04, 0x44, 0x02, 0x42, 0x06, 0x46, 0x01, 0x41];
const ONES: &[u8] = &[0x00, 0x80, 0x20, 0xA0, 0x10, 0x90, 0x30, 0xB0, 0x08, 0x88];

lazy_static! {
    // Encode
    static ref ENCODE_NLBB: Vec<(u8, u32)> = style_encode(*NLBB);
    static ref ENCODE_NLBT: Vec<(u8, u32)> = style_encode(*NLBT);
    static ref ENCODE_NRBB: Vec<(u8, u32)> = style_encode(*NRBB);
    static ref ENCODE_NRBT: Vec<(u8, u32)> = style_encode(*NRBT);

    // Decode
    static ref DECODE_NLBB: Vec<(u8, u8)> = style_decode(*NLBB);
    static ref DECODE_NLBT: Vec<(u8, u8)> = style_decode(*NLBT);
    static ref DECODE_NRBB: Vec<(u8, u8)> = style_decode(*NRBB);
    static ref DECODE_NRBT: Vec<(u8, u8)> = style_decode(*NRBT);
}

pub type EncodeFn = fn(u8) -> char;
pub type DecodeFn = fn(char) -> u8;

/**
Translate a [`u8`] to binary representation using the `bcd` encoding

`bcd`: binary coded decimal

Tens | Ones
---|---
n/a | 8
64 | 4
32 | 2
16 | 1

```
use bbd_lib::*;

assert_eq!(
    (0..=99).map(|x| encode_bcd(x)).collect::<String>(),
    "\
        ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊\
        ⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉\
    ",
);

// All byte values greater than 99 panic:
assert!(
    (100..=255)
        .all(|x| std::panic::catch_unwind(|| encode_bcd(x)).is_err()),
);
```
*/
pub fn encode_bcd(decimal: u8) -> char {
    if decimal > 99 {
        panic!("Invalid BCD value: {decimal}! Must be in range `0..=99`.")
    }
    let d = decimal as usize;
    let tens = d / 10;
    let ones = d - tens * 10;
    char::from_u32(0x2800 + (TENS[tens] as u32) + (ONES[ones] as u32)).unwrap()
}

/**
Translate a binary representation to [`u8`] using the `bcd` encoding

`bcd`: binary coded decimal

Tens | Ones
---|---
n/a | 8
64 | 4
32 | 2
16 | 1

```
use bbd_lib::*;

assert_eq!(
    "\
        ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊\
        ⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉\
    ".chars().map(|x| decode_bcd(x)).collect::<Vec<u8>>(),
    (0..=99).collect::<Vec<u8>>(),
);
```
*/
pub fn decode_bcd(c: char) -> u8 {
    let b = ((c as u32) - 0x2800) as u8;
    let mut decimal: usize = 0;
    for (i, tens) in TENS.iter().enumerate().rev() {
        if b & *tens == *tens {
            decimal += i * 10;
            break;
        }
    }
    for (i, ones) in ONES.iter().enumerate().rev() {
        if b & *ones == *ones {
            decimal += i;
            break;
        }
    }
    decimal as u8
}

/**
Translate a [`u8`] to binary representation using the `direct` encoding

Left | Right
---|---
1 | 8
2 | 16
4 | 32
64 | 128

```
use bbd_lib::*;

assert_eq!(
    (0..=255).map(|x| encode_direct(x)).collect::<String>(),
    "\
        ⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿\
        ⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟⡠⡡⡢⡣⡤⡥⡦⡧⡨⡩⡪⡫⡬⡭⡮⡯⡰⡱⡲⡳⡴⡵⡶⡷⡸⡹⡺⡻⡼⡽⡾⡿\
        ⢀⢁⢂⢃⢄⢅⢆⢇⢈⢉⢊⢋⢌⢍⢎⢏⢐⢑⢒⢓⢔⢕⢖⢗⢘⢙⢚⢛⢜⢝⢞⢟⢠⢡⢢⢣⢤⢥⢦⢧⢨⢩⢪⢫⢬⢭⢮⢯⢰⢱⢲⢳⢴⢵⢶⢷⢸⢹⢺⢻⢼⢽⢾⢿\
        ⣀⣁⣂⣃⣄⣅⣆⣇⣈⣉⣊⣋⣌⣍⣎⣏⣐⣑⣒⣓⣔⣕⣖⣗⣘⣙⣚⣛⣜⣝⣞⣟⣠⣡⣢⣣⣤⣥⣦⣧⣨⣩⣪⣫⣬⣭⣮⣯⣰⣱⣲⣳⣴⣵⣶⣷⣸⣹⣺⣻⣼⣽⣾⣿\
    ",
);
```
*/
pub fn encode_direct(b: u8) -> char {
    char::from_u32(0x2800 + (b as u32)).unwrap()
}

/**
Translate a binary representation to a [`u8`] using the `direct` encoding

Left | Right
---|---
1 | 8
2 | 16
4 | 32
64 | 128

```
use bbd_lib::*;

assert_eq!(
    "\
        ⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿\
        ⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟⡠⡡⡢⡣⡤⡥⡦⡧⡨⡩⡪⡫⡬⡭⡮⡯⡰⡱⡲⡳⡴⡵⡶⡷⡸⡹⡺⡻⡼⡽⡾⡿\
        ⢀⢁⢂⢃⢄⢅⢆⢇⢈⢉⢊⢋⢌⢍⢎⢏⢐⢑⢒⢓⢔⢕⢖⢗⢘⢙⢚⢛⢜⢝⢞⢟⢠⢡⢢⢣⢤⢥⢦⢧⢨⢩⢪⢫⢬⢭⢮⢯⢰⢱⢲⢳⢴⢵⢶⢷⢸⢹⢺⢻⢼⢽⢾⢿\
        ⣀⣁⣂⣃⣄⣅⣆⣇⣈⣉⣊⣋⣌⣍⣎⣏⣐⣑⣒⣓⣔⣕⣖⣗⣘⣙⣚⣛⣜⣝⣞⣟⣠⣡⣢⣣⣤⣥⣦⣧⣨⣩⣪⣫⣬⣭⣮⣯⣰⣱⣲⣳⣴⣵⣶⣷⣸⣹⣺⣻⣼⣽⣾⣿\
    ".chars().map(|x| decode_direct(x)).collect::<Vec<u8>>(),
    (0..=255).collect::<Vec<u8>>(),
);
*/
pub fn decode_direct(c: char) -> u8 {
    ((c as u32) - 0x2800) as u8
}

/**
Core of `encode_{nlbb,nlbt,nrbb,nrbt}` functions
*/
fn encode_nb(b: u8, values: &[(u8, u32)]) -> char {
    char::from_u32(values.iter().fold(
        0x2800,
        |s, (from, to)| {
            if b & *from == *from {
                s + *to
            } else {
                s
            }
        },
    ))
    .unwrap()
}

/**
Core of `decode_{nlbb,nlbt,nrbb,nrbt}` functions
*/
fn decode_nb(c: char, values: &[(u8, u8)]) -> u8 {
    let b = ((c as u32) - 0x2800) as u8;
    values.iter().fold(
        0,
        |s, (from, to)| {
            if b & *from == *from {
                s + *to
            } else {
                s
            }
        },
    )
}

/**
Translate a [`u8`] to binary representation using the `nlbb` encoding

`nlbb`: most significant nibble left, most significant bit bottom

Left | Right
---|---
16 | 1
32 | 2
64 | 4
128 | 8

```
use bbd_lib::*;

assert_eq!(
    (0..=255).map(|x| encode_nlbb(x)).collect::<String>(),
    "\
        ⠀⠈⠐⠘⠠⠨⠰⠸⢀⢈⢐⢘⢠⢨⢰⢸⠁⠉⠑⠙⠡⠩⠱⠹⢁⢉⢑⢙⢡⢩⢱⢹⠂⠊⠒⠚⠢⠪⠲⠺⢂⢊⢒⢚⢢⢪⢲⢺⠃⠋⠓⠛⠣⠫⠳⠻⢃⢋⢓⢛⢣⢫⢳⢻\
        ⠄⠌⠔⠜⠤⠬⠴⠼⢄⢌⢔⢜⢤⢬⢴⢼⠅⠍⠕⠝⠥⠭⠵⠽⢅⢍⢕⢝⢥⢭⢵⢽⠆⠎⠖⠞⠦⠮⠶⠾⢆⢎⢖⢞⢦⢮⢶⢾⠇⠏⠗⠟⠧⠯⠷⠿⢇⢏⢗⢟⢧⢯⢷⢿\
        ⡀⡈⡐⡘⡠⡨⡰⡸⣀⣈⣐⣘⣠⣨⣰⣸⡁⡉⡑⡙⡡⡩⡱⡹⣁⣉⣑⣙⣡⣩⣱⣹⡂⡊⡒⡚⡢⡪⡲⡺⣂⣊⣒⣚⣢⣪⣲⣺⡃⡋⡓⡛⡣⡫⡳⡻⣃⣋⣓⣛⣣⣫⣳⣻\
        ⡄⡌⡔⡜⡤⡬⡴⡼⣄⣌⣔⣜⣤⣬⣴⣼⡅⡍⡕⡝⡥⡭⡵⡽⣅⣍⣕⣝⣥⣭⣵⣽⡆⡎⡖⡞⡦⡮⡶⡾⣆⣎⣖⣞⣦⣮⣶⣾⡇⡏⡗⡟⡧⡯⡷⡿⣇⣏⣗⣟⣧⣯⣷⣿\
    ",
);
```
*/
pub fn encode_nlbb(b: u8) -> char {
    encode_nb(b, &ENCODE_NLBB)
}

/**
Translate a binary representation to [`u8`] using the `nlbb` encoding

`nlbb`: most significant nibble left, most significant bit bottom

Left | Right
---|---
16 | 1
32 | 2
64 | 4
128 | 8

```
use bbd_lib::*;

assert_eq!(
    "\
        ⠀⠈⠐⠘⠠⠨⠰⠸⢀⢈⢐⢘⢠⢨⢰⢸⠁⠉⠑⠙⠡⠩⠱⠹⢁⢉⢑⢙⢡⢩⢱⢹⠂⠊⠒⠚⠢⠪⠲⠺⢂⢊⢒⢚⢢⢪⢲⢺⠃⠋⠓⠛⠣⠫⠳⠻⢃⢋⢓⢛⢣⢫⢳⢻\
        ⠄⠌⠔⠜⠤⠬⠴⠼⢄⢌⢔⢜⢤⢬⢴⢼⠅⠍⠕⠝⠥⠭⠵⠽⢅⢍⢕⢝⢥⢭⢵⢽⠆⠎⠖⠞⠦⠮⠶⠾⢆⢎⢖⢞⢦⢮⢶⢾⠇⠏⠗⠟⠧⠯⠷⠿⢇⢏⢗⢟⢧⢯⢷⢿\
        ⡀⡈⡐⡘⡠⡨⡰⡸⣀⣈⣐⣘⣠⣨⣰⣸⡁⡉⡑⡙⡡⡩⡱⡹⣁⣉⣑⣙⣡⣩⣱⣹⡂⡊⡒⡚⡢⡪⡲⡺⣂⣊⣒⣚⣢⣪⣲⣺⡃⡋⡓⡛⡣⡫⡳⡻⣃⣋⣓⣛⣣⣫⣳⣻\
        ⡄⡌⡔⡜⡤⡬⡴⡼⣄⣌⣔⣜⣤⣬⣴⣼⡅⡍⡕⡝⡥⡭⡵⡽⣅⣍⣕⣝⣥⣭⣵⣽⡆⡎⡖⡞⡦⡮⡶⡾⣆⣎⣖⣞⣦⣮⣶⣾⡇⡏⡗⡟⡧⡯⡷⡿⣇⣏⣗⣟⣧⣯⣷⣿\
    ".chars().map(|x| decode_nlbb(x)).collect::<Vec<u8>>(),
    (0..=255).collect::<Vec<u8>>());
```
*/
pub fn decode_nlbb(c: char) -> u8 {
    decode_nb(c, &DECODE_NLBB)
}

/**
Translate a [`u8`] to binary representation using the `nlbt` encoding

`nlbt`: most significant nibble left, most significant bit top

Left | Right
---|---
128 | 8
64 | 4
32 | 2
16 | 1

```
use bbd_lib::*;

assert_eq!(
    (0..=255).map(|x| encode_nlbt(x)).collect::<String>(),
    "\
        ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⠨⢨⠘⢘⠸⢸⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⡨⣨⡘⣘⡸⣸⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⠬⢬⠜⢜⠼⢼⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⡬⣬⡜⣜⡼⣼\
        ⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊⠪⢪⠚⢚⠺⢺⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⡪⣪⡚⣚⡺⣺⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⠮⢮⠞⢞⠾⢾⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⡮⣮⡞⣞⡾⣾\
        ⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⠩⢩⠙⢙⠹⢹⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉⡩⣩⡙⣙⡹⣹⠅⢅⠥⢥⠕⢕⠵⢵⠍⢍⠭⢭⠝⢝⠽⢽⡅⣅⡥⣥⡕⣕⡵⣵⡍⣍⡭⣭⡝⣝⡽⣽\
        ⠃⢃⠣⢣⠓⢓⠳⢳⠋⢋⠫⢫⠛⢛⠻⢻⡃⣃⡣⣣⡓⣓⡳⣳⡋⣋⡫⣫⡛⣛⡻⣻⠇⢇⠧⢧⠗⢗⠷⢷⠏⢏⠯⢯⠟⢟⠿⢿⡇⣇⡧⣧⡗⣗⡷⣷⡏⣏⡯⣯⡟⣟⡿⣿\
    ",
);
```
*/
pub fn encode_nlbt(b: u8) -> char {
    encode_nb(b, &ENCODE_NLBT)
}

/**
Translate a binary representation to [`u8`] using the `nlbt` encoding

`nlbt`: most significant nibble left, most significant bit top

Left | Right
---|---
128 | 8
64 | 4
32 | 2
16 | 1

```
use bbd_lib::*;

assert_eq!(
    "\
        ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⠨⢨⠘⢘⠸⢸⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⡨⣨⡘⣘⡸⣸⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⠬⢬⠜⢜⠼⢼⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⡬⣬⡜⣜⡼⣼\
        ⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊⠪⢪⠚⢚⠺⢺⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⡪⣪⡚⣚⡺⣺⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⠮⢮⠞⢞⠾⢾⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⡮⣮⡞⣞⡾⣾\
        ⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⠩⢩⠙⢙⠹⢹⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉⡩⣩⡙⣙⡹⣹⠅⢅⠥⢥⠕⢕⠵⢵⠍⢍⠭⢭⠝⢝⠽⢽⡅⣅⡥⣥⡕⣕⡵⣵⡍⣍⡭⣭⡝⣝⡽⣽\
        ⠃⢃⠣⢣⠓⢓⠳⢳⠋⢋⠫⢫⠛⢛⠻⢻⡃⣃⡣⣣⡓⣓⡳⣳⡋⣋⡫⣫⡛⣛⡻⣻⠇⢇⠧⢧⠗⢗⠷⢷⠏⢏⠯⢯⠟⢟⠿⢿⡇⣇⡧⣧⡗⣗⡷⣷⡏⣏⡯⣯⡟⣟⡿⣿\
    ".chars().map(|x| decode_nlbt(x)).collect::<Vec<u8>>(),
    (0..=255).collect::<Vec<u8>>());
```
*/
pub fn decode_nlbt(c: char) -> u8 {
    decode_nb(c, &DECODE_NLBT)
}

/**
Translate a [`u8`] to binary representation using the `nrbb` encoding

`nrbb`: most significant nibble right, most significant bit bottom

Left | Right
---|---
1 | 16
2 | 32
4 | 64
8 | 128

```
use bbd_lib::*;

assert_eq!(
    (0..=255).map(|x| encode_nrbb(x)).collect::<String>(),
    "\
        ⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟\
        ⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿\
        ⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟\
        ⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿\
    ",
);
```
*/
pub fn encode_nrbb(b: u8) -> char {
    encode_nb(b, &ENCODE_NRBB)
}

/**
Translate a binary representation to [`u8`] using the `nrbb` encoding

`nrbb`: most significant nibble right, most significant bit bottom

Left | Right
---|---
1 | 16
2 | 32
4 | 64
8 | 128

```
use bbd_lib::*;

assert_eq!(
    "\
        ⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟\
        ⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿\
        ⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟\
        ⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿\
    ".chars().map(|x| decode_nrbb(x)).collect::<Vec<u8>>(),
    (0..=255).collect::<Vec<u8>>(),
);
```
*/
pub fn decode_nrbb(c: char) -> u8 {
    decode_nb(c, &DECODE_NRBB)
}

/**
Translate a [`u8`] to binary representation using the `nrbt` encoding

`nrbt`: most significant nibble right, most significant bit top

Left | Right
---|---
8 | 128
4 | 64
2 | 32
1 | 16

```
use bbd_lib::*;

assert_eq!(
    (0..=255).map(|x| encode_nrbt(x)).collect::<String>(),
    "\
        ⠀⡀⠄⡄⠂⡂⠆⡆⠁⡁⠅⡅⠃⡃⠇⡇⢀⣀⢄⣄⢂⣂⢆⣆⢁⣁⢅⣅⢃⣃⢇⣇⠠⡠⠤⡤⠢⡢⠦⡦⠡⡡⠥⡥⠣⡣⠧⡧⢠⣠⢤⣤⢢⣢⢦⣦⢡⣡⢥⣥⢣⣣⢧⣧\
        ⠐⡐⠔⡔⠒⡒⠖⡖⠑⡑⠕⡕⠓⡓⠗⡗⢐⣐⢔⣔⢒⣒⢖⣖⢑⣑⢕⣕⢓⣓⢗⣗⠰⡰⠴⡴⠲⡲⠶⡶⠱⡱⠵⡵⠳⡳⠷⡷⢰⣰⢴⣴⢲⣲⢶⣶⢱⣱⢵⣵⢳⣳⢷⣷\
        ⠈⡈⠌⡌⠊⡊⠎⡎⠉⡉⠍⡍⠋⡋⠏⡏⢈⣈⢌⣌⢊⣊⢎⣎⢉⣉⢍⣍⢋⣋⢏⣏⠨⡨⠬⡬⠪⡪⠮⡮⠩⡩⠭⡭⠫⡫⠯⡯⢨⣨⢬⣬⢪⣪⢮⣮⢩⣩⢭⣭⢫⣫⢯⣯\
        ⠘⡘⠜⡜⠚⡚⠞⡞⠙⡙⠝⡝⠛⡛⠟⡟⢘⣘⢜⣜⢚⣚⢞⣞⢙⣙⢝⣝⢛⣛⢟⣟⠸⡸⠼⡼⠺⡺⠾⡾⠹⡹⠽⡽⠻⡻⠿⡿⢸⣸⢼⣼⢺⣺⢾⣾⢹⣹⢽⣽⢻⣻⢿⣿\
    ",
);
```
*/
pub fn encode_nrbt(b: u8) -> char {
    encode_nb(b, &ENCODE_NRBT)
}

/**
Translate a binary representation to [`u8`] using the `nrbt` encoding

`nrbt`: most significant nibble right, most significant bit top

Left | Right
---|---
8 | 128
4 | 64
2 | 32
1 | 16

```
use bbd_lib::*;

assert_eq!(
    "\
        ⠀⡀⠄⡄⠂⡂⠆⡆⠁⡁⠅⡅⠃⡃⠇⡇⢀⣀⢄⣄⢂⣂⢆⣆⢁⣁⢅⣅⢃⣃⢇⣇⠠⡠⠤⡤⠢⡢⠦⡦⠡⡡⠥⡥⠣⡣⠧⡧⢠⣠⢤⣤⢢⣢⢦⣦⢡⣡⢥⣥⢣⣣⢧⣧\
        ⠐⡐⠔⡔⠒⡒⠖⡖⠑⡑⠕⡕⠓⡓⠗⡗⢐⣐⢔⣔⢒⣒⢖⣖⢑⣑⢕⣕⢓⣓⢗⣗⠰⡰⠴⡴⠲⡲⠶⡶⠱⡱⠵⡵⠳⡳⠷⡷⢰⣰⢴⣴⢲⣲⢶⣶⢱⣱⢵⣵⢳⣳⢷⣷\
        ⠈⡈⠌⡌⠊⡊⠎⡎⠉⡉⠍⡍⠋⡋⠏⡏⢈⣈⢌⣌⢊⣊⢎⣎⢉⣉⢍⣍⢋⣋⢏⣏⠨⡨⠬⡬⠪⡪⠮⡮⠩⡩⠭⡭⠫⡫⠯⡯⢨⣨⢬⣬⢪⣪⢮⣮⢩⣩⢭⣭⢫⣫⢯⣯\
        ⠘⡘⠜⡜⠚⡚⠞⡞⠙⡙⠝⡝⠛⡛⠟⡟⢘⣘⢜⣜⢚⣚⢞⣞⢙⣙⢝⣝⢛⣛⢟⣟⠸⡸⠼⡼⠺⡺⠾⡾⠹⡹⠽⡽⠻⡻⠿⡿⢸⣸⢼⣼⢺⣺⢾⣾⢹⣹⢽⣽⢻⣻⢿⣿\
    ".chars().map(|x| decode_nrbt(x)).collect::<Vec<u8>>(),
    (0..=255).collect::<Vec<u8>>(),
);
```
*/
pub fn decode_nrbt(c: char) -> u8 {
    decode_nb(c, &DECODE_NRBT)
}

/**
Encode bytes to binary representation using the given function with optional wrapping

```
use bbd_lib::*;

let bcd_content = (0..=99).collect::<Vec<u8>>();
let bytes_content = (0..=255).collect::<Vec<u8>>();
let cases: Vec<(&[u8], EncodeFn, &str)> = vec![
    (
        &bcd_content,
        encode_bcd,
        "\
            ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊\
            ⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉\
        ",
    ),
    (
        &bytes_content,
        encode_direct,
        "\
            ⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿\
            ⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟⡠⡡⡢⡣⡤⡥⡦⡧⡨⡩⡪⡫⡬⡭⡮⡯⡰⡱⡲⡳⡴⡵⡶⡷⡸⡹⡺⡻⡼⡽⡾⡿\
            ⢀⢁⢂⢃⢄⢅⢆⢇⢈⢉⢊⢋⢌⢍⢎⢏⢐⢑⢒⢓⢔⢕⢖⢗⢘⢙⢚⢛⢜⢝⢞⢟⢠⢡⢢⢣⢤⢥⢦⢧⢨⢩⢪⢫⢬⢭⢮⢯⢰⢱⢲⢳⢴⢵⢶⢷⢸⢹⢺⢻⢼⢽⢾⢿\
            ⣀⣁⣂⣃⣄⣅⣆⣇⣈⣉⣊⣋⣌⣍⣎⣏⣐⣑⣒⣓⣔⣕⣖⣗⣘⣙⣚⣛⣜⣝⣞⣟⣠⣡⣢⣣⣤⣥⣦⣧⣨⣩⣪⣫⣬⣭⣮⣯⣰⣱⣲⣳⣴⣵⣶⣷⣸⣹⣺⣻⣼⣽⣾⣿\
        ",
    ),
    (
        &bytes_content,
        encode_nlbb,
        "\
            ⠀⠈⠐⠘⠠⠨⠰⠸⢀⢈⢐⢘⢠⢨⢰⢸⠁⠉⠑⠙⠡⠩⠱⠹⢁⢉⢑⢙⢡⢩⢱⢹⠂⠊⠒⠚⠢⠪⠲⠺⢂⢊⢒⢚⢢⢪⢲⢺⠃⠋⠓⠛⠣⠫⠳⠻⢃⢋⢓⢛⢣⢫⢳⢻\
            ⠄⠌⠔⠜⠤⠬⠴⠼⢄⢌⢔⢜⢤⢬⢴⢼⠅⠍⠕⠝⠥⠭⠵⠽⢅⢍⢕⢝⢥⢭⢵⢽⠆⠎⠖⠞⠦⠮⠶⠾⢆⢎⢖⢞⢦⢮⢶⢾⠇⠏⠗⠟⠧⠯⠷⠿⢇⢏⢗⢟⢧⢯⢷⢿\
            ⡀⡈⡐⡘⡠⡨⡰⡸⣀⣈⣐⣘⣠⣨⣰⣸⡁⡉⡑⡙⡡⡩⡱⡹⣁⣉⣑⣙⣡⣩⣱⣹⡂⡊⡒⡚⡢⡪⡲⡺⣂⣊⣒⣚⣢⣪⣲⣺⡃⡋⡓⡛⡣⡫⡳⡻⣃⣋⣓⣛⣣⣫⣳⣻\
            ⡄⡌⡔⡜⡤⡬⡴⡼⣄⣌⣔⣜⣤⣬⣴⣼⡅⡍⡕⡝⡥⡭⡵⡽⣅⣍⣕⣝⣥⣭⣵⣽⡆⡎⡖⡞⡦⡮⡶⡾⣆⣎⣖⣞⣦⣮⣶⣾⡇⡏⡗⡟⡧⡯⡷⡿⣇⣏⣗⣟⣧⣯⣷⣿\
        ",
    ),
    (
        &bytes_content,
        encode_nlbt,
        "\
            ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⠨⢨⠘⢘⠸⢸⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⡨⣨⡘⣘⡸⣸⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⠬⢬⠜⢜⠼⢼⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⡬⣬⡜⣜⡼⣼\
            ⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊⠪⢪⠚⢚⠺⢺⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⡪⣪⡚⣚⡺⣺⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⠮⢮⠞⢞⠾⢾⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⡮⣮⡞⣞⡾⣾\
            ⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⠩⢩⠙⢙⠹⢹⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉⡩⣩⡙⣙⡹⣹⠅⢅⠥⢥⠕⢕⠵⢵⠍⢍⠭⢭⠝⢝⠽⢽⡅⣅⡥⣥⡕⣕⡵⣵⡍⣍⡭⣭⡝⣝⡽⣽\
            ⠃⢃⠣⢣⠓⢓⠳⢳⠋⢋⠫⢫⠛⢛⠻⢻⡃⣃⡣⣣⡓⣓⡳⣳⡋⣋⡫⣫⡛⣛⡻⣻⠇⢇⠧⢧⠗⢗⠷⢷⠏⢏⠯⢯⠟⢟⠿⢿⡇⣇⡧⣧⡗⣗⡷⣷⡏⣏⡯⣯⡟⣟⡿⣿\
        ",
    ),
    (
        &bytes_content,
        encode_nrbb,
        "\
            ⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟\
            ⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿\
            ⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟\
            ⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿\
        ",
    ),
    (
        &bytes_content,
        encode_nrbt,
        "\
            ⠀⡀⠄⡄⠂⡂⠆⡆⠁⡁⠅⡅⠃⡃⠇⡇⢀⣀⢄⣄⢂⣂⢆⣆⢁⣁⢅⣅⢃⣃⢇⣇⠠⡠⠤⡤⠢⡢⠦⡦⠡⡡⠥⡥⠣⡣⠧⡧⢠⣠⢤⣤⢢⣢⢦⣦⢡⣡⢥⣥⢣⣣⢧⣧\
            ⠐⡐⠔⡔⠒⡒⠖⡖⠑⡑⠕⡕⠓⡓⠗⡗⢐⣐⢔⣔⢒⣒⢖⣖⢑⣑⢕⣕⢓⣓⢗⣗⠰⡰⠴⡴⠲⡲⠶⡶⠱⡱⠵⡵⠳⡳⠷⡷⢰⣰⢴⣴⢲⣲⢶⣶⢱⣱⢵⣵⢳⣳⢷⣷\
            ⠈⡈⠌⡌⠊⡊⠎⡎⠉⡉⠍⡍⠋⡋⠏⡏⢈⣈⢌⣌⢊⣊⢎⣎⢉⣉⢍⣍⢋⣋⢏⣏⠨⡨⠬⡬⠪⡪⠮⡮⠩⡩⠭⡭⠫⡫⠯⡯⢨⣨⢬⣬⢪⣪⢮⣮⢩⣩⢭⣭⢫⣫⢯⣯\
            ⠘⡘⠜⡜⠚⡚⠞⡞⠙⡙⠝⡝⠛⡛⠟⡟⢘⣘⢜⣜⢚⣚⢞⣞⢙⣙⢝⣝⢛⣛⢟⣟⠸⡸⠼⡼⠺⡺⠾⡾⠹⡹⠽⡽⠻⡻⠿⡿⢸⣸⢼⣼⢺⣺⢾⣾⢹⣹⢽⣽⢻⣻⢿⣿\
        ",
    ),
];
for (content, convert_byte, result) in cases {
    assert_eq!(encode(content, convert_byte, 0, 0), result);
}
```
*/
pub fn encode(
    content: &[u8],
    convert_byte: EncodeFn,
    columns: usize,
    prev_content_length: usize,
) -> String {
    let wrapping = columns > 0;
    let mut c = if wrapping {
        prev_content_length - (prev_content_length / columns) * columns
    } else {
        0
    };

    let mut r = String::with_capacity(4 * content.len());
    for b in content {
        r.push(convert_byte(*b));

        if wrapping {
            c += 1;
            if c >= columns {
                r.push_str("\\\n");
                c = 0;
            }
        }
    }

    r
}

/**
Decode binary representation to bytes using the given function with optional wrapping

```
use bbd_lib::*;

let bcd_content = (0..=99).collect::<Vec<u8>>();
let bytes_content = (0..=255).collect::<Vec<u8>>();
let cases: Vec<(&str, DecodeFn, &[u8])> = vec![
    (
        "\
            ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊\
            ⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉\
        ",
        decode_bcd,
        &bcd_content,
    ),
    (
        "\
            ⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿\
            ⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟⡠⡡⡢⡣⡤⡥⡦⡧⡨⡩⡪⡫⡬⡭⡮⡯⡰⡱⡲⡳⡴⡵⡶⡷⡸⡹⡺⡻⡼⡽⡾⡿\
            ⢀⢁⢂⢃⢄⢅⢆⢇⢈⢉⢊⢋⢌⢍⢎⢏⢐⢑⢒⢓⢔⢕⢖⢗⢘⢙⢚⢛⢜⢝⢞⢟⢠⢡⢢⢣⢤⢥⢦⢧⢨⢩⢪⢫⢬⢭⢮⢯⢰⢱⢲⢳⢴⢵⢶⢷⢸⢹⢺⢻⢼⢽⢾⢿\
            ⣀⣁⣂⣃⣄⣅⣆⣇⣈⣉⣊⣋⣌⣍⣎⣏⣐⣑⣒⣓⣔⣕⣖⣗⣘⣙⣚⣛⣜⣝⣞⣟⣠⣡⣢⣣⣤⣥⣦⣧⣨⣩⣪⣫⣬⣭⣮⣯⣰⣱⣲⣳⣴⣵⣶⣷⣸⣹⣺⣻⣼⣽⣾⣿\
        ",
        decode_direct,
        &bytes_content,
    ),
    (
        "\
            ⠀⠈⠐⠘⠠⠨⠰⠸⢀⢈⢐⢘⢠⢨⢰⢸⠁⠉⠑⠙⠡⠩⠱⠹⢁⢉⢑⢙⢡⢩⢱⢹⠂⠊⠒⠚⠢⠪⠲⠺⢂⢊⢒⢚⢢⢪⢲⢺⠃⠋⠓⠛⠣⠫⠳⠻⢃⢋⢓⢛⢣⢫⢳⢻\
            ⠄⠌⠔⠜⠤⠬⠴⠼⢄⢌⢔⢜⢤⢬⢴⢼⠅⠍⠕⠝⠥⠭⠵⠽⢅⢍⢕⢝⢥⢭⢵⢽⠆⠎⠖⠞⠦⠮⠶⠾⢆⢎⢖⢞⢦⢮⢶⢾⠇⠏⠗⠟⠧⠯⠷⠿⢇⢏⢗⢟⢧⢯⢷⢿\
            ⡀⡈⡐⡘⡠⡨⡰⡸⣀⣈⣐⣘⣠⣨⣰⣸⡁⡉⡑⡙⡡⡩⡱⡹⣁⣉⣑⣙⣡⣩⣱⣹⡂⡊⡒⡚⡢⡪⡲⡺⣂⣊⣒⣚⣢⣪⣲⣺⡃⡋⡓⡛⡣⡫⡳⡻⣃⣋⣓⣛⣣⣫⣳⣻\
            ⡄⡌⡔⡜⡤⡬⡴⡼⣄⣌⣔⣜⣤⣬⣴⣼⡅⡍⡕⡝⡥⡭⡵⡽⣅⣍⣕⣝⣥⣭⣵⣽⡆⡎⡖⡞⡦⡮⡶⡾⣆⣎⣖⣞⣦⣮⣶⣾⡇⡏⡗⡟⡧⡯⡷⡿⣇⣏⣗⣟⣧⣯⣷⣿\
        ",
        decode_nlbb,
        &bytes_content,
    ),
    (
        "\
            ⠀⢀⠠⢠⠐⢐⠰⢰⠈⢈⠨⢨⠘⢘⠸⢸⡀⣀⡠⣠⡐⣐⡰⣰⡈⣈⡨⣨⡘⣘⡸⣸⠄⢄⠤⢤⠔⢔⠴⢴⠌⢌⠬⢬⠜⢜⠼⢼⡄⣄⡤⣤⡔⣔⡴⣴⡌⣌⡬⣬⡜⣜⡼⣼\
            ⠂⢂⠢⢢⠒⢒⠲⢲⠊⢊⠪⢪⠚⢚⠺⢺⡂⣂⡢⣢⡒⣒⡲⣲⡊⣊⡪⣪⡚⣚⡺⣺⠆⢆⠦⢦⠖⢖⠶⢶⠎⢎⠮⢮⠞⢞⠾⢾⡆⣆⡦⣦⡖⣖⡶⣶⡎⣎⡮⣮⡞⣞⡾⣾\
            ⠁⢁⠡⢡⠑⢑⠱⢱⠉⢉⠩⢩⠙⢙⠹⢹⡁⣁⡡⣡⡑⣑⡱⣱⡉⣉⡩⣩⡙⣙⡹⣹⠅⢅⠥⢥⠕⢕⠵⢵⠍⢍⠭⢭⠝⢝⠽⢽⡅⣅⡥⣥⡕⣕⡵⣵⡍⣍⡭⣭⡝⣝⡽⣽\
            ⠃⢃⠣⢣⠓⢓⠳⢳⠋⢋⠫⢫⠛⢛⠻⢻⡃⣃⡣⣣⡓⣓⡳⣳⡋⣋⡫⣫⡛⣛⡻⣻⠇⢇⠧⢧⠗⢗⠷⢷⠏⢏⠯⢯⠟⢟⠿⢿⡇⣇⡧⣧⡗⣗⡷⣷⡏⣏⡯⣯⡟⣟⡿⣿\
        ",
        decode_nlbt,
        &bytes_content,
    ),
    (
        "\
            ⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟\
            ⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿\
            ⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟\
            ⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿\
        ",
        decode_nrbb,
        &bytes_content,
    ),
    (
        "\
            ⠀⡀⠄⡄⠂⡂⠆⡆⠁⡁⠅⡅⠃⡃⠇⡇⢀⣀⢄⣄⢂⣂⢆⣆⢁⣁⢅⣅⢃⣃⢇⣇⠠⡠⠤⡤⠢⡢⠦⡦⠡⡡⠥⡥⠣⡣⠧⡧⢠⣠⢤⣤⢢⣢⢦⣦⢡⣡⢥⣥⢣⣣⢧⣧\
            ⠐⡐⠔⡔⠒⡒⠖⡖⠑⡑⠕⡕⠓⡓⠗⡗⢐⣐⢔⣔⢒⣒⢖⣖⢑⣑⢕⣕⢓⣓⢗⣗⠰⡰⠴⡴⠲⡲⠶⡶⠱⡱⠵⡵⠳⡳⠷⡷⢰⣰⢴⣴⢲⣲⢶⣶⢱⣱⢵⣵⢳⣳⢷⣷\
            ⠈⡈⠌⡌⠊⡊⠎⡎⠉⡉⠍⡍⠋⡋⠏⡏⢈⣈⢌⣌⢊⣊⢎⣎⢉⣉⢍⣍⢋⣋⢏⣏⠨⡨⠬⡬⠪⡪⠮⡮⠩⡩⠭⡭⠫⡫⠯⡯⢨⣨⢬⣬⢪⣪⢮⣮⢩⣩⢭⣭⢫⣫⢯⣯\
            ⠘⡘⠜⡜⠚⡚⠞⡞⠙⡙⠝⡝⠛⡛⠟⡟⢘⣘⢜⣜⢚⣚⢞⣞⢙⣙⢝⣝⢛⣛⢟⣟⠸⡸⠼⡼⠺⡺⠾⡾⠹⡹⠽⡽⠻⡻⠿⡿⢸⣸⢼⣼⢺⣺⢾⣾⢹⣹⢽⣽⢻⣻⢿⣿\
        ",
        decode_nrbt,
        &bytes_content,
    ),
];
for (content, convert_byte, result) in cases {
    assert_eq!(decode(content, convert_byte), result);
}
```
*/
pub fn decode(content: &str, convert_char: DecodeFn) -> Vec<u8> {
    let mut r = Vec::with_capacity(content.len() / 4);
    for c in content.chars() {
        if !['\\', '\n'].contains(&c) {
            r.push(convert_char(c));
        }
    }
    r
}

/**
Process a style definition into a list of from/to conversion values for encoding
*/
fn style_encode(values: [u32; 8]) -> Vec<(u8, u32)> {
    values
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (1 << i, v))
        .collect()
}

/**
Process a style definition into a list of from/to conversion values for decoding
*/
fn style_decode(values: [u32; 8]) -> Vec<(u8, u8)> {
    values
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v as u8, 1 << i))
        .collect()
}
