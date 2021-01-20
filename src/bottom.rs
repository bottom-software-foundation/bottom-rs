use std::collections::HashMap;
use lazy_static::lazy_static;
use maplit::hashmap;
use std::error::Error;
use std::fmt;

lazy_static! {
    static ref CHARACTER_VALUES: HashMap<u8, &'static str> = hashmap! {
        200 => "🫂" ,
        50 => "💖",
        10 => "✨",
        5 => "🥺",
        1 => ",",
        0 => "❤️",
    };
    static ref BYTE_TO_EMOJI: HashMap<u8, String> = {
        let mut m = HashMap::new();
        for i in 0..=255 {
            m.insert(i, byte_to_emoji(i));
        }
        m
    };
    static ref EMOJI_TO_BYTE: HashMap<&'static str, &'static u8> = {
        let mut m = HashMap::new();
        for (byte, emoji) in &mut BYTE_TO_EMOJI.iter() {
            m.insert(emoji.as_str().trim_end_matches("👉👈"), byte);
        }
        m
    };
}

#[derive(Debug)]
pub struct TranslationError {
    pub why: String,
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl Error for TranslationError {}

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

pub fn encode_byte(value: u8) -> &'static str {
    &BYTE_TO_EMOJI[&value]
}

pub fn decode_byte(input: &dyn AsRef<str>) -> Result<u8, TranslationError> {
    let input_ref = input.as_ref();
    let result = EMOJI_TO_BYTE.get(input_ref).ok_or(TranslationError {
        why: format!("Cannot decode character {}", input_ref),
    })?;
    Ok(**result)
}

pub fn encode_string(input: &dyn AsRef<str>) -> String {
    input.as_ref().bytes().map(encode_byte).collect::<String>()
}

pub fn decode_string(input: &dyn AsRef<str>) -> Result<String, TranslationError> {
    let input = input.as_ref();
    let result = {
        // Older versions used a ZWSP as a character separator, instead of `👉👈`.
        if input.contains("\u{200B}") {
            input.trim_end_matches("\u{200B}").split("\u{200B}")
        } else {
            input.trim_end_matches("👉👈").split("👉👈")
        }
    }
    .map(|c| decode_byte(&c))
    .collect::<Result<Vec<u8>, _>>()?;

    Ok(String::from_utf8_lossy(&result).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_encode() {
        assert_eq!(
            encode_string(&"Test"),
            "💖✨✨✨,,,,👉👈💖💖,👉👈💖💖✨🥺👉👈💖💖✨🥺,👉👈"
        );
    }

    #[test]
    fn test_byte_encode() {
        assert_eq!(encode_byte(b'h'), "💖💖,,,,👉👈",);
    }

    #[test]
    fn test_char_decode() {
        assert_eq!(decode_byte(&"💖💖,,,,").unwrap(), b'h',);
    }

    #[test]
    fn test_string_decode() {
        // Test that we haven't killed backwards-compat
        assert_eq!(
            decode_string(&"💖✨✨✨,,,,\u{200B}💖💖,\u{200B}💖💖✨🥺\u{200B}💖💖✨🥺,\u{200B}")
                .unwrap(),
            "Test"
        );
        assert_eq!(
            decode_string(&"💖✨✨✨,,,,👉👈💖💖,👉👈💖💖✨🥺👉👈💖💖✨🥺,👉👈").unwrap(),
            "Test"
        );
    }

    #[test]
    fn test_unicode_string_encode() {
        assert_eq!(
            encode_string(&"🥺"),
            "🫂✨✨✨✨👉👈💖💖💖🥺,,,,👉👈💖💖💖✨🥺👉👈💖💖💖✨✨✨🥺,👉👈"
        );
        assert_eq!(
            encode_string(&"がんばれ"),
            "🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈💖💖✨✨✨✨👉👈🫂✨✨🥺,,👉👈\
            💖💖✨✨✨👉👈💖💖✨✨✨✨🥺,,👉👈🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈\
            💖💖💖✨✨🥺,👉👈🫂✨✨🥺,,👉👈💖💖✨✨✨👉👈💖💖✨✨✨✨👉👈"
        );
    }

    #[test]
    fn test_unicode_string_decode() {
        assert_eq!(
            decode_string(&"🫂✨✨✨✨👉👈💖💖💖🥺,,,,👉👈💖💖💖✨🥺👉👈💖💖💖✨✨✨🥺,👉👈")
                .unwrap(),
            "🥺",
        );
        assert_eq!(
            decode_string(
                &"🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈💖💖✨✨✨✨👉👈🫂✨✨🥺,,👉👈\
            💖💖✨✨✨👉👈💖💖✨✨✨✨🥺,,👉👈🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈\
            💖💖💖✨✨🥺,👉👈🫂✨✨🥺,,👉👈💖💖✨✨✨👉👈💖💖✨✨✨✨👉👈"
            )
            .unwrap(),
            "がんばれ",
        );
    }
}
