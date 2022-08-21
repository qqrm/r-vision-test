fn replase_p_to_asterisk(text_chunk: &mut Vec<u8>) {
    let mut i = 0;

    while i != text_chunk.len() - 1 {
        if text_chunk[i] == 208 && text_chunk[i + 1] == 191 {
            text_chunk[i] = 42;
            text_chunk[i + 1] = 42;
        }
        i += 1;
    }
}

fn replase_a_to_asterisk(text_chunk: &mut Vec<u8>) {
    for ch in text_chunk {
        if *ch == b'a' {
            *ch = b'*';
        }
    }
}

pub fn process_text(text_chunk: &mut Vec<u8>) -> anyhow::Result<()> {
    replase_p_to_asterisk(text_chunk);
    replase_a_to_asterisk(text_chunk);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{replase_a_to_asterisk, replase_p_to_asterisk};

    #[test]
    fn test_replace_a() {
        let str = "all grab gorilla".to_owned();

        let mut vec: Vec<u8> = Vec::from(str.as_bytes());

        replase_a_to_asterisk(&mut vec);

        let new_str = String::from_utf8(vec).unwrap();

        assert_eq!(new_str, "*ll gr*b gorill*");
    }

    #[test]
    fn test_replace_p() {
        let str = "привет пока".to_owned();

        let mut vec: Vec<u8> = Vec::from(str.as_bytes());

        replase_p_to_asterisk(&mut vec);

        let new_str = String::from_utf8(vec).unwrap();

        assert_eq!(new_str, "**ривет **ока");
    }
}
