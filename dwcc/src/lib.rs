mod lexer;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_read() {
        let contents: String = utils::get_file_contents("fixtures/return_2.c");
        assert!(contents.starts_with("int main()"))
    }
}
