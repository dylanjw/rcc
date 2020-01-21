use std::boxed::Box;

mod lexer;
mod utils;

#[derive(Debug)]
struct Expression {
    value: lexer::Token,
}

#[derive(Debug)]
struct Function <'a>{
    name: lexer::Token,
    signature: &'a[lexer::Token],
    contents: Vec<AstNode>,
}

#[derive(Debug)]
enum NodeType {
    Return(Expression),
    Function(Vec<lexer::Token>, Box<AstNode>)
}

#[derive(Debug)]
struct AstNode {
    NodeType: NodeType,
    pos: usize,
}

struct Parser {
    pos: usize,
    ast: Vec<AstNode>,
    tokens: Vec<lexer::Token>,
}


impl Parser {
    fn new() -> Parser {
        Parser { pos: 0, ast: vec!(), tokens: vec!() }
    }
    fn token(&self) -> &lexer::Token {
        &self.tokens[self.pos]
    }
    fn bump(&mut self) {
        self.pos = self.pos + 1
    }
    fn parse_from_tokens(mut self, tokens: &[lexer::Token]) -> &Parser {

        while self.pos < tokens.len() {
            match self.token() {
                lexer::Token::KeywordInt => {
                    let fn_pos = self.pos;
                    self.eat_to_token(&lexer::Token::OpenBrace);
                    let signature = &self.tokens[fn_pos+1..self.pos];

                    let fn_expression_pos = self.pos;
                    self.eat_to_token(&lexer::Token::CloseBrace);

                    let contents = &self.tokens[fn_expression_pos..self.pos];
                    let fn_parser = Parser::new();
                    fn_parser.parse_from_tokens(&contents);

                    let func = Function { name: signature[0], signature: signature, contents: fn_parser.ast };
                }
                lexer::Token::KeywordReturn => {
                    let ret_pos = self.pos;
                    self.eat_to_token(&lexer::Token::Semicolon);
                    // Later there will be more complicated expressions
                    // Currently only integers are supported
                    let expression = &self.tokens[ret_pos + 1..self.pos];
                        match expression[0] {
                            lexer::Token::Integer(v) => {
                                self.ast.push(
                                    AstNode {
                                        NodeType: NodeType::Return(Expression{value: expression[0]}),
                                        pos: ret_pos,
                                    });

                                self.bump();
                            },
                            _ => {},
                    }
                },
                _ => panic!("something got effed"),
            }
        }
        &self
    }
    fn eat_to_token(&mut self, t: &lexer::Token) {
        self.eat_while(|x| x != t);
    }
    fn eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(&lexer::Token) -> bool,
    {
        let mut eaten: usize = 0;
        while predicate(self.token()) {
            eaten += 1;
            self.bump();
        }
        eaten
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_read() {
        let contents: String = utils::get_file_contents("fixtures/return_2.c");
        assert!(contents.starts_with("int main()"))
    }

    #[test]
    fn test_lexer() {
        let contents: String = utils::get_file_contents("fixtures/return_2.c");
        println!("{:?}", lexer::tokenize(&contents));
    }

    #[test]
    fn test_parser() {
        let contents: String = utils::get_file_contents("fixtures/return_2.c");
        let tokens = lexer::tokenize(&contents);
        let ast = Parser::new().parse_from_tokens().ast;
        println!("{:?}", ast);
    }
}
