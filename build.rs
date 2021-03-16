use lazy_static::lazy_static;
use maplit::hashmap;
use phf_codegen;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

lazy_static! {
    static ref CHARACTER_VALUES: HashMap<u8, &'static str> = hashmap! {
        200 => "🫂" ,
        50 => "💖",
        10 => "✨",
        5 => "🥺",
        1 => ",",
        0 => "❤️",
    };
    static ref BYTE_TO_EMOJI: [String; 256] = {
        const EMPTY_STRING: String = String::new();

        let mut m = [EMPTY_STRING; 256];
        for i in 0..=255u8 {
            m[i as usize] = byte_to_emoji(i);
        }
        m
    };
}

fn byte_to_emoji(value: u8) -> String {
    let mut buffer = String::new();
    let mut value = value;

    if value == 0 {
        return CHARACTER_VALUES[&0].to_string();
    }

    loop {
        let (to_push, subtract_by) = {
            if value >= 200 {
                (CHARACTER_VALUES[&200], 200)
            } else if value >= 50 {
                (CHARACTER_VALUES[&50], 50)
            } else if value >= 10 {
                (CHARACTER_VALUES[&10], 10)
            } else if value >= 5 {
                (CHARACTER_VALUES[&5], 5)
            } else if value >= 1 {
                (CHARACTER_VALUES[&1], 1)
            } else {
                break;
            }
        };

        buffer.push_str(to_push);
        value -= subtract_by;
    }

    buffer.push_str("👉👈");
    buffer
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("maps.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static BYTE_TO_EMOJI: [&'static str; 256] = [").unwrap();

    for emoji in BYTE_TO_EMOJI.iter() {
        write!(&mut file, "\"{}\",", emoji).unwrap();
    }

    write!(&mut file, "];\n").unwrap();

    write!(
        &mut file,
        "static EMOJI_TO_BYTE: phf::Map<&'static str, u8> = "
    )
    .unwrap();

    let mut m = phf_codegen::Map::new();

    for (byte, emoji) in BYTE_TO_EMOJI.iter().enumerate() {
        m.entry(emoji.as_str().trim_end_matches("👉👈"), &byte.to_string());
    }

    write!(&mut file, "{}", m.build()).unwrap();
    write!(&mut file, ";\n").unwrap();
}
