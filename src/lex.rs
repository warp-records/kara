use crate::vm::VmError;
use strum_macros::FromRepr;
use TokenType::*;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub line_num: usize,
    //How to make this an iterator over
    pub content: &'a str,
}

#[derive(Debug, FromRepr, Copy, Clone, PartialEq)]
#[repr(u8)]
//Parse rule function table requires that the order
//here be kept as is since their values are used as indices
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character s.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals; Might change the implementation of these later
    //to utilize the way Clox stores literals
    Identifier,
    Str,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    //Use Blank instead of None
    Newline,
    Eof,
    Blank,

}

pub fn lex(source: &str) -> Result<Vec<Token>, VmError> {
    let mut line_num = 0;
    let mut iter = source.chars().peekable();

    let mut tokens = Vec::new();
    let mut curr_idx: usize = 0;

    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            if c == '\n' {
                line_num += 1;
            }
            curr_idx += 1;
            continue;
        }

        assert_eq!(source.as_bytes()[curr_idx] as char, c);

        //Handle compile time errors later
        //who needs error handling anyways
        let start_idx = curr_idx;

        let token_type = match c {
            '>' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    curr_idx += 1;
                    GreaterEqual
                }
                _ => Greater,
            },

            '<' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    curr_idx += 1;
                    LessEqual
                }
                _ => Less,
            },

            '=' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    curr_idx += 1;
                    EqualEqual
                }
                _ => Equal,
            },

            '!' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    curr_idx += 1;
                    BangEqual
                }
                _ => Bang,
            },

            '+' => Plus,
            '-' => Minus,
            '*' => Star,

            '/' => match iter.peek() {
                Some(&'/') => {
                    while iter.next() != Some('\n') {}
                    line_num += 1;
                    continue;
                }

                _ => Slash,
            },

            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            ';' => Semicolon,

            '\"' => {
                iter.next();
                curr_idx += 1;

                //Might crash if you get to the end
                let mut chr = iter.next();
                while chr.is_some() && chr != Some('\"') {
                    curr_idx += 1;
                    chr = iter.next();
                }

                curr_idx += 1;

                //Do nothing with it for now

                if chr.is_none() {
                    panic!(
                        "Hahaha sucker, not gonna tell you what the error here is, \
                    		fuck you and good luck debugging lmao"
                    );
                }

                Str
            }

            c if c.is_ascii_digit() => {
                while let Some(next) = iter.peek() {
                    //in case code ends with number
                    if next.to_digit(10).is_none() {
                        break;
                    };

                    iter.next();
                    curr_idx += 1;
                }

                if iter.peek() == Some(&'.') {
                    iter.next();
                    curr_idx += 1;
                }

                while let Some(next) = iter.peek() {
                    if next.to_digit(10).is_none() {
                        break;
                    };

                    iter.next();
                    curr_idx += 1;
                }

                Number
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut lexeme = String::from(c);

                while let Some(c) = iter.peek() {
                    if (*c).is_alphabetic() || (*c) == '_' {
                        lexeme.push(*c);
                        iter.next();
                        curr_idx += 1;
                    } else {
                        break;
                    }
                }

                match lexeme.as_str() {
                    "and" => And,
                    "class" => Class,
                    "else" => Else,
                    "false" => False,
                    "for" => For,
                    "fun" => Fun,
                    "if" => If,
                    "nil" => Nil,
                    "or" => Or,
                    "print" => Print,
                    "return" => Return,
                    "super" => Super,
                    "this" => This,
                    "true" => True,
                    "var" => Var,
                    "while" => While,
                    //idfk how we're gonna handle strings lol
                    _ => Identifier,
                }
            }

            _ => panic!(),
        };

        curr_idx += 1;

        let token = Token {
            kind: token_type,
            line_num: line_num,
            content: &source[start_idx..curr_idx],
        };

        println!("{}\t{:?}", token.content, token.kind);

        tokens.push(token);
    }

    //Must alter once you start reading into multiple chunks
    tokens.push(Token {
        kind: Eof,
        line_num: line_num,
        content: "",
    });

    Ok(tokens)
}
