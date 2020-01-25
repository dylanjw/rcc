use std::fs::File;
use std::io::Read;

pub fn get_file_contents(filename: &str) -> String {
    let mut file = File::open(filename).expect("Could not read file.");
    let mut contents = String::new();

    file.read_to_string(&mut contents);

    contents
}

pub fn is_numeric(c: char) -> bool {
    ('0' <= c && c <= '9')
}


pub fn is_id_start(c: char) -> bool {
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || c == '_'
}

pub fn is_id_continue(c: char) -> bool {
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || ('0' <= c && c <= '9')
        || c == '_'
}

pub fn is_special_char(v: &str) -> bool {
    if v == "" {
        return false;
    }
    let specialchar = "{}();";
    for c in v.chars().into_iter() {
        if !specialchar.contains(c) {
            return false;
        }
    }
    true
}

pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    match c {
        // Usual ASCII suspects
        | '\u{0009}' // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
            => true,
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_not_numeric() {
        assert!(!is_numeric(' '))
    }

    #[test]
    fn test_alpha_not_numeric() {
        assert!(!is_numeric('a'))
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric('1'))
    }

    #[test]
    fn test_not_alphanumeric() {
        assert!(!is_numeric(' '))
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_numeric('1'))
    }
}
