#[derive(Debug, PartialEq)]
pub enum TokenKind {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    KeywordInt,
    KeywordReturn,
    Identifier,
    Integer,
    Error,
    None,
}

pub struct Token {
    kind: TokenKind,
    pos: usize,
    len: usize,
}

fn is_numeric(v: &str) -> bool {
    if v == "" {
        return false;
    }
    let numeric = "0987654321";
    for c in v.chars().into_iter() {
        if !numeric.contains(c) {
            return false;
        }
    }
    true
}

fn is_alphanumeric(v: &str) -> bool {
    if v == "" {
        return false;
    }
    let alphanumeric = concat!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
        "0987654321"
    );
    for c in v.chars().into_iter() {
        if !alphanumeric.contains(c) {
            return false;
        }
    }
    true
}

fn is_special_char(v: &str) -> bool {
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

pub struct Cursor {
    src: &str,

}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut cursor = 0;
    let mut output = vec!();
    let length = input.chars().count();

    while cursor < length {
        let mut token = String::new();
        if is_special_char(&input[cursor..=cursor]) {
            token.push_str(&input[cursor..=cursor]);
            cursor += 1;
        } else if is_alphanumeric(&input[cursor..=cursor]) {
            while is_alphanumeric(&input[cursor..=cursor]) {
                token.push_str(&input[cursor..=cursor]);
                cursor += 1;
            }
        } else {
            cursor += 1;
            continue
        }
        let assigned = assign_token(&token);
        println!("Assigning \"{}\" to {:?}", &token, &assigned);
        output.push(assigned);
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emptystring_not_numeric() {
        assert!(!is_numeric(""))
    }

    #[test]
    fn test_space_not_numeric() {
        assert!(!is_numeric(" "))
    }

    #[test]
    fn test_alpha_not_numeric() {
        assert!(!is_numeric("a"))
    }

    #[test]
    fn test_emptystring_not_alphanumeric() {
        assert!(!is_alphanumeric(""))
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("1"))
    }

    #[test]
    fn test_not_alphanumeric() {
        assert!(!is_numeric(" "))
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_numeric("1"))
    }
}
