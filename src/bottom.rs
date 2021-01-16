use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref CHARACTER_VALUES: HashMap<&'static str, u8> = hashmap! {
        "🫂" => 200,
        "💖" => 50,
        "✨" => 10,
        "🥺" => 5,
        "," => 1,
        "❤️" => 0,
    };

    static ref CHARACTER_VALUES_REVERSED: HashMap<u8, &'static str> = hashmap! {
        200 => "🫂" ,
        50 => "💖",
        10 => "✨",
        5 => "🥺",
        1 => ",",
        0 => "❤️",
    };
}

#[derive(Debug)]
pub struct TranslationError {
    pub why: String
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl Error for TranslationError {}

pub fn encode_string(input: &dyn AsRef<str>) -> String {
    input
        .as_ref()
        .chars()
        .map(encode_char)
        .collect::<String>()
}

pub fn encode_char(char: char) -> String {
    let mut buffer = String::new();
    let mut value = char as u8;

    if value == 0 {
        return CHARACTER_VALUES_REVERSED[&0].to_string();
    }

    loop {
        let (to_push, subtract_by) = match value {
            200..=255 => (CHARACTER_VALUES_REVERSED[&200], 200),
            50..=199 => (CHARACTER_VALUES_REVERSED[&50], 50),
            10..=49 => (CHARACTER_VALUES_REVERSED[&10], 10),
            5..=9 => (CHARACTER_VALUES_REVERSED[&5], 5),
            1..=4 => (CHARACTER_VALUES_REVERSED[&1], 1),
            _ => break
        };

        buffer.push_str(to_push);
        value -= subtract_by;
    };

    buffer.push_str("👉👈");
    buffer
}

pub fn decode_string(input: &dyn AsRef<str>) -> Result<String, TranslationError> {
    let r = input.as_ref();
    {
        if r.contains("\u{200B}") {
            r.trim_end_matches("\u{200B}")
            .split("\u{200B}")
        }
        else {
            r.trim_end_matches("👉👈")
            .split("👉👈")
        }
    }
    .map(|c| decode_char(&c))
    .collect::<Result<String, _>>()
}

pub fn decode_char(input: &dyn AsRef<str>) -> Result<char, TranslationError> {
    let result = input
        .as_ref()
        .graphemes(true)
        .map(|g| {
            CHARACTER_VALUES
                .get(g)
                .ok_or(TranslationError {
                    why: format!("Cannot decode character {}", g)
                })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<u8>();

    Ok(result as char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_encode() {
        assert_eq!(
            encode_string(&"Test"),
            "💖✨✨✨,,,,👉👈💖💖,👉👈💖💖✨🥺👉👈💖💖✨🥺,👉👈".to_string()
        );
    }

    #[test]
    fn test_char_encode() {
        assert_eq!(
            encode_char('h'),
            "💖💖,,,,👉👈".to_string(),
        );
    }

    #[test]
    fn test_string_decode() {
        assert_eq!(
            decode_string(&"💖✨✨✨,,,,\u{200B}💖💖,\u{200B}💖💖✨🥺\u{200B}💖💖✨🥺,\u{200B}").unwrap(),
            "Test".to_string()
        );
    }

    #[test]
    fn test_char_decode() {
        assert_eq!(
            decode_char(&"💖💖,,,,").unwrap(),
            'h',
        );
    }
}
