use std::hash::Hash;
use std::fmt::{self, format};
use std::{fs::File, io::Write, vec};
use std::collections::HashSet;

use crate::lex::Token;
use crate::parse::Parser;
use crate::{lex::TokenType, parse::Node};

pub struct Emitter {
    file_path: String,
    header: String,
    code: String,
    ast: Node,
    stack: Vec<Node>,
    symbols: HashSet<String>
}

impl Emitter {
    pub fn new(file: String, ast: Node) -> Self {
        Emitter {
            file_path: file,
            header: String::from(""),
            code: String::from(""),
            ast: ast,
            stack: vec![],
            symbols: HashSet::new()
        }
    }

    pub fn print_tree(&mut self) {
        self.header_line("#include <stdio.h>\n");
        self.header_line("int main(void) {");

        self.stack.push(self.ast.clone());
        self.rprint_tree();

        self.emit_line("return 0;");
        self.emit_line("}");

        self.write_out();
    }

    fn rprint_tree(&mut self) {
        // println!("{}", self.token.text);

        let node = self.stack.last().unwrap().clone();

        match node.token.kind {
            TokenType::PRINT => {
                if node.children.len() != 1 {
                    unreachable!("Expected vector length equal to 1, got {}", node.children.len());
                }

                let child_node = &node.children[0];
                if child_node.token.kind == TokenType::STRING {
                    self.emit_line(&("printf(\"".to_owned() + &child_node.token.text + "\\n\");"));
                } else {
                    self.emit("printf(\"%.2f\\n\", (float)(");

                    self.stack.push(child_node.clone());
                    self.rprint_tree();
                    
                    self.emit_line("));");
                }
                self.stack.pop();
                return;
            },
            TokenType::IF => {
                self.emit("if (");
                for child in &node.children {
                    self.stack.push(child.clone());
                    self.rprint_tree();
                    self.stack.pop();
                }
                self.emit_line(") {");
                return;
            },
            TokenType::WHILE => {
                self.emit("while (");

                self.stack.push(node.children[0].clone());
                self.rprint_tree();
                self.stack.pop();

                self.emit_line(") {");
                for child in &node.children[1..] {
                    self.stack.push(child.clone());
                    self.rprint_tree();
                    self.stack.pop();
                }
                self.emit_line("}");
                return;
            },
            TokenType::LABEL => {
                self.emit_line(&(node.token.text.clone() + ":"));
            },
            TokenType::GOTO => {
                self.emit_line(&("goto ".to_owned() + &node.token.text.clone() + ";"));
            },
            TokenType::LET => {
                let first_node = node.children[0].clone();
                let identifier = first_node.token.text;
                if !self.symbols.contains(&identifier) {
                    self.header_line(&format!("float {};", &identifier));
                    self.symbols.insert(identifier.clone());
                }

                self.emit(&format!("{} = ", identifier));

                for child in &node.children[1..] {
                    self.stack.push(child.clone());
                    self.rprint_tree();
                    self.stack.pop();
                }
                self.emit_line(";");
                return;
            },
            TokenType::INPUT => {
                let first_node = node.children[0].clone();
                let identifier = first_node.token.text;
                if !self.symbols.contains(&identifier) {
                    self.header_line(&format!("float {};", &identifier));
                    self.symbols.insert(identifier.clone());
                }

                self.emit_line(&("if (0 == scanf(\"%".to_owned() + "f\", &" + &identifier + ")) {"));
                self.emit_line(&(identifier + " = 0;"));
                self.emit("scanf(\"%");
                self.emit_line("*s\");");
                self.emit_line("}");
                return;
            },
            TokenType::COMPARISON => {
                for child in &node.children {
                    let text = child.token.text.clone();
                    if Parser::is_comparison(&text) {
                        self.emit(&format!(" {} ", text));
                    } else {
                        self.stack.push(child.clone());
                        self.rprint_tree();
                        self.stack.pop();
                    }
                }
                return;
            },
            TokenType::EXPRESSION => {
                for child in &node.children {
                    let kind = child.token.kind;
                    if kind == TokenType::PLUS || kind == TokenType::MINUS {
                        self.emit(&format!(" {} ", child.token.text.clone()));
                    } else {
                        self.stack.push(child.clone());
                        self.rprint_tree();
                        self.stack.pop();
                    }
                }
                return;
            },
            TokenType::TERM => {
                for child in &node.children {
                    let kind = child.token.kind;
                    if kind == TokenType::SLASH || kind == TokenType::ASTERISK {
                        self.emit(&format!(" {} ", child.token.text.clone()));
                    } else {
                        self.stack.push(child.clone());
                        self.rprint_tree();
                        self.stack.pop();
                    }
                }
                return;
            },
            TokenType::PLUS | TokenType::MINUS => {
                self.emit(&format!("{}", node.token.text.clone()));
            },
            TokenType::NUMBER | TokenType::IDENT => {
                self.emit(&format!("{}", node.token.text.clone()));
            }
            _ => {}
        }

        // let message: &str = node.string.as_str();
        // match message {
        //     "comparison" => {
        //         if node.children.len() != 2 {
        //             unreachable!("Expected comparison to have two children! Got {}", node.children.len());
        //         }

        //         self.stack.push(node.children[0].clone());
        //         self.rprint_tree();
        //         self.stack.pop();

        //         self.emit(&format!(" {} ", node.token.text.clone()));

        //         self.stack.push(node.children[1].clone());
        //         self.rprint_tree();
        //         self.stack.pop();
        //     }
        //     _ => {}
        // }

        for child in &node.children {
            self.stack.push(child.clone());
            self.rprint_tree();
            self.stack.pop();
        }
    }


    fn emit(&mut self, line: &str) {
        self.code += line;
    }

    fn emit_line(&mut self, line: &str) {
        self.emit(line);
        self.code += "\n";
    }

    fn header_line(&mut self, line: &str) {
        self.header += line;
        self.header += "\n";
    }

    fn write_out(&mut self) {
        let mut file = File::create(self.file_path.clone()).expect("Unable to open file!");
        let _ok = write!(file, "{}", self.header).unwrap();
        // let _ok = write!(file, "{}", "\n").unwrap();
        let _ok = write!(file, "{}", self.code).unwrap();
    }

}