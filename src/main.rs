mod lex;

fn main() {
    let source = "IF+-123 foo*THEN/";
    let mut lexer = lex::Lexer::new(source.to_string());
    let mut token = lexer.get_token();
    while token.kind != lex::TokenType::EOF {
        println!("{0} {1}", token.kind, token.text);
        token = lexer.get_token();
    }
}

fn test1() {
    let source = "LET foobar = 123";
    let mut lexer = lex::Lexer::new(source.to_string());
    while lexer.peek() != '\0' {
        println!("{0}", lexer.cur_char);
        lexer.next_char();
    }
}

fn test2() {
    let source = "+- */ >>= = !=";
    let mut lexer = lex::Lexer::new(source.to_string());
    let mut token = lexer.get_token();
    while token.kind != lex::TokenType::EOF {
        println!("{0}", token.kind);
        token = lexer.get_token();
    }
}

fn test3() {
    let source = "+- \"This is a string\" # This is a comment!\n */";
    let mut lexer = lex::Lexer::new(source.to_string());
    let mut token = lexer.get_token();
    while token.kind != lex::TokenType::EOF {
        println!("{0} {1}", token.kind, token.text);
        token = lexer.get_token();
    }
}

fn test4() {
    let source = "+-123 9.8654*/";
    let mut lexer = lex::Lexer::new(source.to_string());
    let mut token = lexer.get_token();
    while token.kind != lex::TokenType::EOF {
        println!("{0} {1}", token.kind, token.text);
        token = lexer.get_token();
    }
}