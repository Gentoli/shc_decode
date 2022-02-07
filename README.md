# SHC Decoder

Decodes single chunk [SMART Health Cards](https://spec.smarthealth.cards/) QR code. 

## Requirements

rustc >= 1.56.0 (`let-else` feature)

## Usage

``` sh
$ shc_decode "./QR_code_screenshot.png"
full jwt: ...
header: {...}
payload: {...}
```