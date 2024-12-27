use crate::lex::Lexer;
use crate::lex::Token;
use crate::lex::TokenType;

use std::collections::HashSet;
use std::vec;

#[derive(Clone)]
pub struct Node {
    pub string: String,
    pub token: Token,
    pub children: Vec<Node>,
}


impl Node {
    pub fn new() -> Self {
        let node = Node {
            string: "Root".to_string(),
            token: Token{text: "".to_string(), kind: TokenType::BAD},
            children: vec![]
        };
        node
    }

    pub fn print_tree(&self, level: usize) {
        // Print the current node's token text with indentation
        println!("{}{}", "  ".repeat(level), self.token.text);

        // Recursively print all children nodes
        for child in &self.children {
            child.print_tree(level + 1);
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
    ast: Node
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
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_gotoed: HashSet::new(),
            ast: Node::new()
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn program(&mut self) -> Node {
        println!("PROGRAM");

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(TokenType::EOF) {
            let sub_node = self.statement();
            self.ast.children.push(sub_node);
        }

        for label in &self.labels_gotoed {
            if !self.labels_declared.contains(label) {
                unreachable!("Attempting to GOTO undeclared label: {label}");
            }
        }

        self.ast.clone()
    }

    fn statement(&mut self) -> Node {
        // "PRINT" (expression | string)

        let mut node = Node {
            string: String::from("statement"),
            token: self.cur_token.clone(),
            children: vec![]
        };

        if self.check_token(TokenType::PRINT) {
            println!("STATEMENT-PRINT");

            self.next_token();
            if self.check_token(TokenType::STRING) {
                node.children.push(Node { string: "print-value".to_string(), token: self.cur_token.clone(), children: vec![] });
                self.next_token();
            } else {
                // expect expression
                node.children.push(self.expression());
            }
        } else if self.check_token(TokenType::IF) {
            // | "IF" comparison "THEN" nl {statement} "ENDIF" nl
            println!("STATEMENT-IF");

            self.next_token();
            // node.children.push(self.comparison());
            node.children.push(self.comparison());


            self.match_token(TokenType::THEN);
            self.nl();

            while !self.check_token(TokenType::ENDIF) {
                // node.children.push(self.statement());
                node.children.push(self.statement());
            }
            self.match_token(TokenType::ENDIF);

        } else if self.check_token(TokenType::WHILE) {
            // | "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
            println!("STATEMENT-WHILE");

            self.next_token();
            node.children.push(self.comparison());

            self.match_token(TokenType::REPEAT);
            self.nl();

            while !self.check_token(TokenType::ENDWHILE) {
                node.children.push(self.statement());
            }
            self.match_token(TokenType::ENDWHILE);

        } else if self.check_token(TokenType::LABEL) {
            // | "LABEL" ident nl
            println!("STATEMENT-LABEL");

            self.next_token();

            if self.labels_declared.contains(&self.cur_token.text) {
                unreachable!("Label {0} is already declared!", self.cur_token.text);
            }
            self.labels_declared.insert(self.cur_token.text.clone());

            self.match_token(TokenType::IDENT);
            
        } else if self.check_token(TokenType::GOTO) {
            // | "GOTO" ident nl
            println!("STATEMENT-GOTO");

            self.next_token();
            self.labels_gotoed.insert(self.cur_token.text.clone());
            self.match_token(TokenType::IDENT);
            
        } else if self.check_token(TokenType::LET) {
            // | "LET" ident "=" expression nl
            println!("STATEMENT-LET");

            self.next_token();
            
            if !self.symbols.contains(&self.cur_token.text) {
                self.symbols.insert(self.cur_token.text.clone());
            }
            
            node.children.push(Node { string: "assignee".to_string(), token: self.cur_token.clone(), children: vec![] });
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);
            node.children.push(self.expression());
            
        } else if self.check_token(TokenType::INPUT) {
            // | "INPUT" ident nl
            println!("STATEMENT-INPUT");

            self.next_token();

            if !self.symbols.contains(&self.cur_token.text) {
                self.symbols.insert(self.cur_token.text.clone());
            }
            
            node.children.push(Node { string: "assignee".to_string(), token: self.cur_token.clone(), children: vec![] });
            self.match_token(TokenType::IDENT);
            
        } else {
            println!("Not a valid statement! Got {0} of type {1}", self.cur_token.text, self.cur_token.text);
        }

        // newline
        self.nl();
        node
    }

    fn comparison(&mut self) -> Node {
        // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
        println!("COMPARISON");

        let mut node: Node = Node {
            string: String::from("comparison"),
            token: Token { text: String::from("comparison"), kind: TokenType::COMPARISON },
            children: vec![]
        };

        node.children.push(self.expression());


        if !Parser::is_comparison(&self.cur_token.text) {
            unreachable!("Expected comparison token, got {0} instead", self.cur_token.text);
        }
        node.children.push(Node{string: "Equality".to_string(), token: self.cur_token.clone(), children: vec![]});

        self.next_token();
        node.children.push(self.expression());

        while Parser::is_comparison(&self.cur_token.text) {
            node.children.push(Node{string: "Equality".to_string(), token: self.cur_token.clone(), children: vec![]});
            self.next_token();
            node.children.push(self.expression());
        }

        node
    }

    pub fn is_comparison(op: &str) -> bool {
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

    fn expression(&mut self) -> Node {
        // expression ::= term {( "-" | "+" ) term}
        println!("EXPRESSION");

        let mut node = Node {
            string: String::from("expression"),
            token: Token { text: String::from("expression"), kind: TokenType::EXPRESSION },
            children: vec![]
        };

        node.children.push(self.term());
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            node.children.push(Node{string: "plus/minus".to_string(), token: self.cur_token.clone(), children: vec![]});
            self.next_token();
            node.children.push(self.term());
        }

        node
    }

    fn term(&mut self) -> Node {
        // term ::= unary {( "/" | "*" ) unary}
        println!("TERM");

        let mut node = Node {
            string: String::from("term"),
            token: Token { text: String::from("term"), kind: TokenType::TERM },
            children: vec![]
        };

        node.children.push(self.unary());
        while self.check_token(TokenType::SLASH) || self.check_token(TokenType::ASTERISK) {
            node.children.push(Node{string: "mult/div".to_string(), token: self.cur_token.clone(), children: vec![]});
            self.next_token();
            node.children.push(self.unary());
        }

        node
    }

    fn unary(&mut self) -> Node {
        // unary ::= ["+" | "-"] primary
        println!("UNARY");

        let mut old_token = Token { text: "+".to_string(), kind: TokenType::PLUS };

        // optional to handle cases like +2, -3, -3 * +2 etc.
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            old_token = self.cur_token.clone();
            self.next_token();
        }

        let child = self.primary();
        Node {
            string: String::from("unary"),
            token: old_token,
            children: vec![child]
        }
    }

    fn primary(&mut self) -> Node {
        println!("PRIMARY ({0})", self.cur_token.text);
        // primary ::= number | ident

        let old_token = self.cur_token.clone();
        if self.check_token(TokenType::NUMBER) {
            self.next_token();
        } else if self.check_token(TokenType::IDENT) {
            if !self.symbols.contains(&self.cur_token.text) {
                unreachable!("Attempting to reference variable before assignment {0}", self.cur_token.text);
            }
            self.next_token();
        } else {
            unreachable!("Unexpected Primary token of {0}", self.cur_token.text);
        }

        Node {
            string: String::from("primary"),
            token: old_token,
            children: vec![]
        }
    }

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