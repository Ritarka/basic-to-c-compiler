#[allow(dead_code)]

use std::fs;

mod lex;
mod parse;
mod emitter;
use emitter::Emitter;
use parse::Node;
use lex::Token;
use lex::TokenType;


fn main() {
    let source = fs::read_to_string("test7.bas").expect("Could not open file!");
    let lexer = lex::Lexer::new(source.to_string());
    let mut parser = parse::Parser::new(lexer);
    let ast = parser.program();
    println!("Parser completed!");

    ast.print_tree(0);
    let mut emitter = Emitter::new("prog.c".to_string(), ast);
    emitter.print_tree();
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

fn test5() {
    let source = "IF+-123 foo*THEN/";
    let mut lexer = lex::Lexer::new(source.to_string());
    let mut token = lexer.get_token();
    while token.kind != lex::TokenType::EOF {
        println!("{0} {1}", token.kind, token.text);
        token = lexer.get_token();
    }
}

fn test6() {
    // let root = Node {
    //     token: Token {
    //         text: "PROGRAM".to_string(),
    //         kind: TokenType::BAD, // Replace with appropriate TokenType if needed
    //     },
    //     children: vec![
    //         Node {
    //             token: Token {
    //                 text: "PRINT".to_string(),
    //                 kind: TokenType::PRINT,
    //             },
    //             children: vec![
    //                 Node {
    //                     token: Token {
    //                         text: "\"Hello, World!\"".to_string(),
    //                         kind: TokenType::STRING,
    //                     },
    //                     children: vec![],
    //                 },
    //             ],
    //         },
    //         Node {
    //             token: Token {
    //                 text: "LET".to_string(),
    //                 kind: TokenType::LET,
    //             },
    //             children: vec![
    //                 Node {
    //                     token: Token {
    //                         text: "x".to_string(),
    //                         kind: TokenType::IDENT,
    //                     },
    //                     children: vec![],
    //                 },
    //                 Node {
    //                     token: Token {
    //                         text: "5".to_string(),
    //                         kind: TokenType::NUMBER,
    //                     },
    //                     children: vec![],
    //                 },
    //             ],
    //         },
    //     ],
    // };

    // Print the tree structure
    // root.print_tree(0);
}