use std::vec;

use crate::{lex::TokenType, lex::Token, parse::Node};

pub fn optimize(mut root: Node) -> Node {
    // r_collapse_unary(&mut root);
    r_optimize_terms(&mut root);
    // root.print_tree(0);
    r_optimize_expression(&mut root);
    return root;
}

fn r_optimize_terms(node: &mut Node) {
    if node.token.kind == TokenType::TERM {
        // collapse unary
        for child in &mut node.children {
            if child.token.kind == TokenType::ASTERISK || child.token.kind == TokenType::SLASH {
                continue;
            }
            if child.token.kind == TokenType::NUMBER {
                continue;
            }
            let sign = if child.token.kind == TokenType::PLUS { "" } else { "-" };
            let value = child.children[0].token.text.clone();
            if child.children[0].token.kind == TokenType::IDENT {
                child.children.clear();
                child.token = Token{text: (sign.to_owned() + &value), kind: TokenType::IDENT};
            } else {
                let number: i32 = (sign.to_owned() + &value).parse().unwrap();
                child.children.clear();
                child.token = Token{text: number.to_string(), kind: TokenType::NUMBER};                
            }
        }

        let mut skip_next = false;
        let mut new_vec: Vec<Node> = Vec::new();
        
        // if node.children.len() == 1 {
        //     return;
        // }

        for (i, child) in &mut node.children.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            new_vec.push(child.clone());
            
            if child.token.kind != TokenType::ASTERISK && child.token.kind != TokenType::SLASH {
                continue;
            }
            let left = &new_vec[new_vec.len()-2].clone();
            let right = &node.children[i+1];

            if left.token.kind != TokenType::NUMBER || right.token.kind != TokenType::NUMBER {
                continue;
            }

            let left_val: i32 = left.token.text.parse().unwrap();
            let right_val: i32 = right.token.text.parse().unwrap();
            let mut combo: f32;
            if child.token.kind == TokenType::ASTERISK {
                combo = (left_val as f32) * right_val as f32;
            } else {
                combo = (left_val as f32) / right_val as f32;
            }
            new_vec.pop();
            new_vec.pop();

            new_vec.push(Node { token: Token { text: combo.to_string(), kind: TokenType::NUMBER }, children: vec![] });
            skip_next = true;
        }
        node.children = new_vec;
        if node.children.len() == 1 {
            *node = Node{token: node.children[0].token.clone(), children: vec![]};
        }

    } else {
        for child in &mut node.children {
            r_optimize_terms(child);
        }
    }
}

fn r_optimize_expression(node: &mut Node) {
    if node.token.kind == TokenType::EXPRESSION {
        let mut skip_next = false;
        let mut new_vec: Vec<Node> = Vec::new();
        if node.children.len() == 1 {
            return;
        }
        for (i, child) in &mut node.children.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            new_vec.push(child.clone());            
            if child.token.kind != TokenType::PLUS && child.token.kind != TokenType::MINUS {
                continue;
            }
            let left = &new_vec[new_vec.len()-2].clone();
            let right = &node.children[i+1];

            if left.token.kind != TokenType::NUMBER || right.token.kind != TokenType::NUMBER {
                continue;
            }

            let left_val: i32 = left.token.text.parse().unwrap();
            let right_val: i32 = right.token.text.parse().unwrap();
            let mut combo: f32 = 1.0;
            if child.token.kind == TokenType::PLUS {
                combo = (left_val as f32) + right_val as f32;
            } else {
                combo = (left_val as f32) - right_val as f32;
            }
            new_vec.pop();
            new_vec.pop();

            new_vec.push(Node { token: Token { text: combo.to_string(), kind: TokenType::NUMBER }, children: vec![] });
            skip_next = true;
        }
        node.children = new_vec;
        if node.children.len() == 1 {
            *node = Node{token: node.children[0].token.clone(), children: vec![]};
        }

    } else {
        for child in &mut node.children {
            r_optimize_expression(child);
        }
    }
}