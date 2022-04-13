extern crate qrcodegen;
extern crate png;
use qrcodegen::{QrCode, QrCodeEcc};
use png::Encoder;
use std::io::{Read, BufWriter};
use std::fs::File;
use std::env;

const MAX_SIZE: usize = 2953;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut file = File::open(path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let size = file.read_to_end(&mut buf).unwrap();
    println!("File size: {} ({} bits)", size, size * 8);
    if size > MAX_SIZE {
        panic!("Your data does not fit in a QR code.");
    }

    let qr = QrCode::encode_binary(&buf, QrCodeEcc::Low).unwrap();
    println!("{}", qr.size());

    let writepath = &args[2];
    let writefile = File::create(writepath).unwrap();
    let ref mut w = BufWriter::new(writefile);
    write_image(&qr, w);
}

fn write_image(qr: &QrCode, writer: &mut BufWriter<File>) {
    let size: u32 = qr.size() as u32;
    println!("{}", size);

    let mut encoder = Encoder::new(writer, size, size);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_palette(vec![255, 255, 255, 0, 0, 0]);
    encoder.set_depth(png::BitDepth::One);

    encoder.add_text_chunk("text".to_string(), "why hello there!".to_string()).unwrap();

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&qr_image_data(qr)).unwrap();
}

fn qr_image_data(qr: &QrCode) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    let mut bit: u8 = 7;
    let mut current_val: u8 = 0;
    for y in 0 .. qr.size() {
        for x in 0 .. qr.size() {
            let val: u8 = if qr.get_module(x, y) { 1 } else { 0 };
            current_val |= val << bit;
            if bit == 0 {
                vec.push(current_val);
                current_val = 0;
                bit = 8;
            }
            bit -= 1;
        }

        vec.push(current_val);
        current_val = 0;
        bit = 7;
    }
    println!("{}", vec.len());
    vec
}