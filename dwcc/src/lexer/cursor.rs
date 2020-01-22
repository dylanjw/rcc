use std::str::Chars;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    initial_len: usize,
    next: char,
}

impl Cursor<'_> {
    pub(crate) fn new<'a>(input: &'a str) -> Cursor {
        Cursor {
            chars: input.chars(),
            initial_len: input.len(),
            next: '\0' // EOF char
        }
    }

    pub(crate) fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or('\0')
    }

    pub(crate) fn first(&self) -> char {
        self.nth_char(0)
    }

    fn chars(&self) -> Chars {
        self.chars.clone()
    }

    pub(crate) fn bump(&mut self) {
        self.chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new("GUANTANAMERA!");
        assert_eq!(13, cursor.initial_len);
        assert_eq!('G', cursor.first());
        cursor.bump();
        assert_eq!('U', cursor.first());
    }
}
