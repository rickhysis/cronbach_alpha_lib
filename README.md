# cronbach_alpha_lib

run this test 
```
$ cargo test -- --nocapture
```
## Usage
How to called this library and easiest and most efficient method of creating 
``` rust
use cronbach_alpha_lib::reliable;

let data = vec![
    vec![1, 2, 1, 2, 3, 2, 1, 2, 3, 2], // Q1 = question 1
    vec![3, 2, 3, 1, 3, 3, 1, 3, 3, 2], // Q2 = question 2
    vec![3, 2, 3, 2, 3, 3, 1, 2, 3, 2], // Q3 = question 3
    vec![3, 2, 3, 2, 4, 3, 1, 2, 4, 3], // Q4 = question 4
    vec![2, 3, 3, 3, 4, 3, 1, 2, 2, 1], // Q5 = question 5
];
let result: f32 = reliable(&data); // 0.838
assert_eq!(is_reliable(result), true); // is reliable because more than 0.6
```
# Donate
If this repository has been useful to you, please consider saying "thanks" by donating money, even if it's just one cents. This shows your appreciation and keeps me motivated to work on further software improvements.

## Bitcoin
Please send [bitcoin](https://bitcoin.org) to the following address:

[1BBqhnWbr9TBE1d1QXWpD23BiGKQXRQMPb](https://blockstream.info/address/1BBqhnWbr9TBE1d1QXWpD23BiGKQXRQMPb)

![bitcoin QR code](https://raw.githubusercontent.com/giowck/symphytum/master/stuff/donation-resources/bitcoin_qr.png)

## Ethereum
Please send ETH to the following address:

[0x9Be4c638CDC4b7d89b8Ea3720cd1f39E32276E53](https://etherscan.io/address/0x9Be4c638CDC4b7d89b8Ea3720cd1f39E32276E53)

![ETH QR code](https://raw.githubusercontent.com/giowck/symphytum/master/stuff/donation-resources/ethereum_qr.png)