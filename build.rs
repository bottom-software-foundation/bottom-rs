use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// For more information see the bottom spec at
// <https://github.com/bottom-software-foundation/spec>.
fn byte_to_emoji(value: u8) -> String {
    let mut buffer = String::new();
    let mut value = value;

    if value == 0 {
        return "❤️".to_string();
    }

    loop {
        let (emoji, subtract) = match value {
            200..=255 => ("🫂", 200),
            50..=199 => ("💖", 50),
            10..=49 => ("✨", 10),
            5..=9 => ("🥺", 5),
            1..=4 => (",", 1),
            0 => break,
        };

        buffer.push_str(emoji);
        value -= subtract;
    }

    buffer.push_str("👉👈");
    buffer
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("maps.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let bytes_as_emoji = (0..=255).map(byte_to_emoji).collect::<Vec<_>>();

    write!(&mut file, "static BYTE_TO_EMOJI: [&'static str; 256] = [").unwrap();

    for emoji in bytes_as_emoji.iter() {
        write!(&mut file, "\"{}\",", emoji).unwrap();
    }

    write!(&mut file, "];\n").unwrap();

    write!(&mut file, "static EMOJI_TO_BYTE: phf::Map<&'static str, u8> = ").unwrap();

    let mut m = phf_codegen::Map::new();

    for (byte, emoji) in bytes_as_emoji.iter().enumerate() {
        m.entry(emoji.as_str().trim_end_matches("👉👈"), &byte.to_string());
    }

    write!(&mut file, "{}", m.build()).unwrap();
    write!(&mut file, ";\n").unwrap();
}
