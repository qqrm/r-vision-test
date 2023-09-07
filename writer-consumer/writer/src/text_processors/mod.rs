/// Replaces occurrences of the Cyrillic letter 'п' in the UTF-8 encoded byte sequence with an asterisk '*'.
fn replace_p_to_asterisk(text_chunk: &mut [u8]) {
    for i in 0..text_chunk.len() - 1 {
        if text_chunk[i] == 208 && text_chunk[i + 1] == 191 {
            text_chunk[i] = 42;
            text_chunk[i + 1] = 42;
        }
    }
}

/// Replaces occurrences of the letter 'a' with an asterisk '*'.
fn replace_a_to_asterisk(text_chunk: &mut [u8]) {
    for ch in text_chunk.iter_mut() {
        if *ch == b'a' {
            *ch = b'*';
        }
    }
}

/// Applies text transformations.
pub fn process_text(text_chunk: &mut [u8]) -> anyhow::Result<()> {
    replace_p_to_asterisk(text_chunk);
    replace_a_to_asterisk(text_chunk);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{replace_a_to_asterisk, replace_p_to_asterisk};

    #[test]
    fn test_replace_a() {
        let str = "all grab gorilla";
        let mut vec = str.as_bytes().to_vec();
        replace_a_to_asterisk(&mut vec);
        let new_str = String::from_utf8(vec).unwrap();
        assert_eq!(new_str, "*ll gr*b gorill*");
    }

    #[test]
    fn test_replace_p() {
        let str = "привет пока";
        let mut vec = str.as_bytes().to_vec();
        replace_p_to_asterisk(&mut vec);
        let new_str = String::from_utf8(vec).unwrap();
        assert_eq!(new_str, "**ривет **ока");
    }
}
