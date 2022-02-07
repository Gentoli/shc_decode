#![feature(let_else)]

use std::io::Read;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// path to the file with qr code
    filename: String,
}

fn main() {
    let args = Args::parse();

    let img = image::open(args.filename).unwrap();

    let results = bardecoder::default_decoder().decode(&img);

    if results.len() != 1 {
        panic!("no single qr")
    }

    let Some(Ok(data_str)) = results.first() else {
        panic!("no data")
    };

    let data = data_str.strip_prefix("shc:/").unwrap();

    let jwt = data.chars()
        .collect::<Vec<_>>().chunks(2)
        .into_iter().map(|s| {
        if s.len() != 2 {
            panic!("not 2")
        }
        let mut string = String::from(s[0]);
        string.push(s[1]);
        string.parse::<u8>().unwrap()
    }).map(|c| {
        (c + 45) as char
    }).collect::<String>();
    println!("full jwt: {}", jwt);

    let parts = jwt.split('.')
        .map(base64_url::decode)
        .map(|v| v.unwrap()).collect::<Vec<_>>();

    let [header, payload, sig] = parts.as_slice() else {
        panic!("not jwt")
    };

    let header = std::str::from_utf8(header.as_ref()).unwrap();

    let payload = {
        let mut decoder = libflate::deflate::Decoder::new(payload.as_slice());
        let mut payload = String::new();
        decoder.read_to_string(&mut payload).unwrap();
        payload
    };

    println!("header: {}", header);
    println!("payload: {}", payload);
}
