use crate::lex::Lexer;
use crate::lex::Token;
use crate::lex::TokenType;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token
}

/*
program grammer

program ::= {statement}
statement ::= "PRINT" (expression | string) nl
    | "IF" comparison "THEN" nl {statement} "ENDIF" nl
    | "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
    | "LABEL" ident nl
    | "GOTO" ident nl
    | "LET" ident "=" expression nl
    | "INPUT" ident nl
comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
expression ::= term {( "-" | "+" ) term}
term ::= unary {( "/" | "*" ) unary}
unary ::= ["+" | "-"] primary
primary ::= number | ident
nl ::= '\n'+
*/

impl Parser {
    pub fn new(input_lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer: input_lexer,
            cur_token: Token{text: "".to_string(), kind: TokenType::BAD},
            peek_token: Token{text: "".to_string(), kind: TokenType::BAD},
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn program(&mut self) {
        println!("PROGRAM");

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(TokenType::EOF) {
            self.statement();
        }
    }

    fn statement(&mut self) {
        // "PRINT" (expression | string)
        if self.check_token(TokenType::PRINT) {
            println!("STATEMENT-PRINT");

            self.next_token();
            if self.check_token(TokenType::STRING) {
                self.next_token();
            } else {
                // expect expression
                self.expression();
            }
        } else if self.check_token(TokenType::IF) {
            // | "IF" comparison "THEN" nl {statement} "ENDIF" nl
            println!("STATEMENT-IF");

            self.next_token();
            self.comparison();

            self.match_token(TokenType::THEN);
            self.nl();

            while !self.check_token(TokenType::ENDIF) {
                self.statement();
            }
            self.match_token(TokenType::ENDIF);

        } else if self.check_token(TokenType::WHILE) {
            // | "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
            println!("STATEMENT-WHILE");

            self.next_token();
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.nl();

            while !self.check_token(TokenType::ENDWHILE) {
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE);

        } else if self.check_token(TokenType::LABEL) {
            // | "LABEL" ident nl
            println!("STATEMENT-LABEL");

            self.next_token();
            self.match_token(TokenType::IDENT);
            
        } else if self.check_token(TokenType::GOTO) {
            // | "GOTO" ident nl
            println!("STATEMENT-GOTO");

            self.next_token();
            self.match_token(TokenType::IDENT);
            
        } else if self.check_token(TokenType::LET) {
            // | "LET" ident "=" expression nl
            println!("STATEMENT-LET");

            self.next_token();
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);
            self.expression();
            
        } else if self.check_token(TokenType::INPUT) {
            // | "INPUT" ident nl
            println!("STATEMENT-INPUT");

            self.next_token();
            self.match_token(TokenType::IDENT);
            
        } else {
            println!("Not a valid statement! Got {0} of type {1}", self.cur_token.text, self.cur_token.text);
        }

        // newline
        self.nl();
    }

    fn comparison(&mut self) {
        // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
        println!("COMPARISON");
        self.expression();


        if !self.is_comparison(&self.cur_token.text) {
            unreachable!("Expected comparison token, got {0} instead", self.cur_token.text);
        }
        self.next_token();
        self.expression();

        while self.is_comparison(&self.cur_token.text) {
            self.next_token();
            self.expression();
        }
    }

    fn is_comparison(&self, op: &str) -> bool {
        match op {
            "==" => true,
            "!=" => true,
            ">" => true,
            ">=" => true,
            "<" => true,
            "<=" => true,
            _ => false
        }
    }

    fn expression(&mut self) {
        // expression ::= term {( "-" | "+" ) term}
        println!("EXPRESSION");
        self.term();
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
            self.term();
        }
    }

    fn term(&mut self) {
        // term ::= unary {( "/" | "*" ) unary}
        println!("TERM");
        self.unary();
        while self.check_token(TokenType::SLASH) || self.check_token(TokenType::ASTERISK) {
            self.next_token();
            self.unary();
        }
    }

    fn unary(&mut self) {
        println!("UNARY");

        // optional to handle cases like +2, -3, -3 * +2 etc.
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
        }
        self.primary();
    }
    fn primary(&mut self) {
        println!("PRIMARY ({0})", self.cur_token.text);
        if self.check_token(TokenType::NUMBER) || self.check_token(TokenType::IDENT) {
            self.next_token();
        } else {
            unreachable!("Unexpected Primary token of {0}", self.cur_token.text);
        }
    }

// unary ::= ["+" | "-"] primary
// primary ::= number | ident


    fn nl(&mut self) {
        println!("NEWLINE");
        self.match_token(TokenType::NEWLINE);
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    fn match_token(&mut self, token_type: TokenType) {
        if !self.check_token(token_type) {
            unreachable!("Expected {0}, got {1}", token_type, self.cur_token.kind)
        }
        self.next_token();
    }

    fn check_token(&mut self, token_type: TokenType) -> bool {
        self.cur_token.kind == token_type
    }

    fn check_peek(&mut self, token_type: TokenType) -> bool {
        self.peek_token.kind == token_type
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }
}