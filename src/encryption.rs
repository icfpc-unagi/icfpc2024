use crypto::aes::{self, KeySize};
use crypto::blockmodes;
use crypto::buffer;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use std::env;

pub fn encrypt(text: &str) -> String {
    let key = env::var("UNAGI_PASSWORD").expect("UNAGI_PASSWORD not set");
    let mut encryptor = aes::cbc_encryptor(
        KeySize::KeySize256,
        key.as_bytes(),
        &[0; 16],
        blockmodes::PkcsPadding,
    );

    let mut encrypted: Vec<u8> = Vec::new();
    let mut read_buffer = buffer::RefReadBuffer::new(text.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        encrypted.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    encrypted
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn decrypt(encrypted_data: &str) -> String {
    let key = env::var("UNAGI_PASSWORD").expect("UNAGI_PASSWORD not set");
    let mut decryptor = aes::cbc_decryptor(
        KeySize::KeySize256,
        key.as_bytes(),
        &[0; 16],
        blockmodes::PkcsPadding,
    );

    let encrypted_data = encrypted_data
        .as_bytes()
        .chunks(2)
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
        .collect::<Vec<u8>>();

    let mut decrypted = Vec::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        decrypted.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    String::from_utf8(decrypted).unwrap()
}

pub fn get_bearer() -> String {
    let encrypted_data = "e95dba263a71fc374611bd8feec4c849960360b38cc08ac13fe05dd52463b494a9642f19ed59ae7bad2c3872ec8bb274";
    decrypt(encrypted_data)
}
