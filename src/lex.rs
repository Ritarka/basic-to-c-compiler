use std::fmt;

pub struct Lexer {
    source: String,
    pub cur_char: char,
    pub cur_pos: u32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let first_char = source.chars().nth(0).unwrap();
        let lex = Lexer {
            source: source + "\n",
            cur_char: first_char,
            cur_pos: 0
        };
        lex
    }

    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos >= self.source.len().try_into().unwrap() {
            self.cur_char = '\0'; // EOF
        } else {
            self.cur_char = self.source.as_bytes()[self.cur_pos as usize] as char;
        }
    }

    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len().try_into().unwrap() {
            return '\0'; // EOF
        }
        return self.source.as_bytes()[(self.cur_pos + 1) as usize] as char
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let token = match self.cur_char {
            '+' => Token{text: self.cur_char.to_string(), kind: TokenType::PLUS},
            '-' => Token{text: self.cur_char.to_string(), kind: TokenType::MINUS},
            '*' => Token{text: self.cur_char.to_string(), kind: TokenType::ASTERISK},
            '/' => Token{text: self.cur_char.to_string(), kind: TokenType::SLASH},
            '\n' => Token{text: self.cur_char.to_string(), kind: TokenType::NEWLINE},
            '\0' => Token{text: self.cur_char.to_string(), kind: TokenType::EOF},
            '=' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token{text: "==".to_string(), kind: TokenType::EQEQ}
                } else {
                    Token{text: "=".to_string(), kind: TokenType::EQ}
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token{text: ">=".to_string(), kind: TokenType::GTEQ}
                } else {
                    Token{text: ">".to_string(), kind: TokenType::GT}
                }
            },
            '<' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token{text: "<=".to_string(), kind: TokenType::LTEQ}
                } else {
                    Token{text: "<".to_string(), kind: TokenType::LT}
                }
            },
            '!' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token{text: "!=".to_string(), kind: TokenType::NOTEQ}
                } else {
                    unreachable!("Invalid input!");
                }
            },
            '\"' => {
                self.next_char();
                let start_pos = self.cur_pos as usize;
                while self.cur_char != '\"' {
                    if self.cur_char == '\r' || self.cur_char == '\n' || self.cur_char == '\t' 
                    || self.cur_char == '\\' || self.cur_char == '%' {
                        unreachable!("Invalid character in strung!");
                    }
                    self.next_char();
                }
                let end_pos = self.cur_pos as usize;
                let string: &str = &self.source[start_pos..end_pos];
                Token{text: string.to_string(), kind: TokenType::STRING}
            },
            '0'..='9' => {
                let start_pos = self.cur_pos as usize;
                while self.peek().is_ascii_digit() {
                    self.next_char();
                }
                if self.peek() == '.' {
                    self.next_char();
                    if !self.peek().is_ascii_digit() {
                        unreachable!("Must have at least one digit after decimal place")
                    }
                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }    
                }
                let end_pos = (self.cur_pos + 1) as usize;
                let string: &str = &self.source[start_pos..end_pos];
                Token{text: string.to_string(), kind: TokenType::NUMBER}
            },
            'a'..='z' | 'A'..='Z' => {
                let start = self.cur_pos as usize;
                while self.peek().is_alphanumeric() {
                    self.next_char();
                }
                let end = (self.cur_pos + 1) as usize;
                let substring: &str = &self.source[start..end];

                let keyword = Token::check_keyword(substring);
                Token{text: substring.to_string(), kind: keyword}
            }
            _ => Token{text: self.cur_char.to_string(), kind: TokenType::BAD},
        };
        self.next_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        } 
    }

    fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        } 
    }

}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum TokenType {
    EOF,
	NEWLINE,
	NUMBER,
	IDENT,
	STRING,
	
    // Keywords.
	LABEL,
	GOTO,
	PRINT,
	INPUT,
	LET,
	IF,
	THEN,
	ENDIF,
	WHILE,
	REPEAT,
	ENDWHILE,
    
    // Operators.
	EQ,
	PLUS,
	MINUS,
	ASTERISK,
	SLASH,
	EQEQ,
	NOTEQ,
	LT,
	LTEQ,
	GT,
	GTEQ,

    BAD,

    // Grammars
    COMPARISON,
    EXPRESSION,
    TERM,

}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenType::EOF => "EOF",
            TokenType::NEWLINE => "NEWLINE",
            TokenType::NUMBER => "NUMBER",
            TokenType::IDENT => "IDENT",
            TokenType::STRING => "STRING",
            TokenType::LABEL => "LABEL",
            TokenType::GOTO => "GOTO",
            TokenType::PRINT => "PRINT",
            TokenType::INPUT => "INPUT",
            TokenType::LET => "LET",
            TokenType::IF => "IF",
            TokenType::THEN => "THEN",
            TokenType::ENDIF => "ENDIF",
            TokenType::WHILE => "WHILE",
            TokenType::REPEAT => "REPEAT",
            TokenType::ENDWHILE => "ENDWHILE",
            TokenType::EQ => "EQ",
            TokenType::PLUS => "PLUS",
            TokenType::MINUS => "MINUS",
            TokenType::ASTERISK => "ASTERISK",
            TokenType::SLASH => "SLASH",
            TokenType::EQEQ => "EQEQ",
            TokenType::NOTEQ => "NOTEQ",
            TokenType::LT => "LT",
            TokenType::LTEQ => "LTEQ",
            TokenType::GT => "GT",
            TokenType::GTEQ => "GTEQ",
            TokenType::BAD => "BAD",
            TokenType::COMPARISON => "COMPARISON",
            TokenType::EXPRESSION => "EXPRESSION",
            TokenType::TERM => "TERM",
        };
        write!(f, "{}", token_str)
    }
}

#[derive(Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    pub fn check_keyword(string: &str) -> TokenType {
        match string {
            "LABEL" => TokenType::LABEL,
            "GOTO" => TokenType::GOTO,
            "PRINT" => TokenType::PRINT,
            "INPUT" => TokenType::INPUT,
            "LET" => TokenType::LET,
            "IF" => TokenType::IF,
            "THEN" => TokenType::THEN,
            "ENDIF" => TokenType::ENDIF,
            "WHILE" => TokenType::WHILE,
            "REPEAT" => TokenType::REPEAT,
            "ENDWHILE" => TokenType::ENDWHILE,
            _ => TokenType::IDENT,
        }   
    }
}