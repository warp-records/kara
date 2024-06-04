
use crate::TokenType::*;
use Op::*;
use strum_macros::FromRepr;
use std::fmt;
use std::fs;
use arrayvec::ArrayVec;
use std::env;

macro_rules! binary_op {
    ($stack:expr, $op:tt) => {{
        let b = $stack.pop().unwrap();
        let a = $stack.pop().unwrap();
        $stack.push(a $op b);
    }};
}

fn main() {

    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let source = fs::read_to_string(&args[1])
            .expect("Error: unable to read file");

        //println!("{}", source);

        let tokens = lex(&source);
        //println!("{:?}", tokens);
    }

    //println!("{:?}", vm.interpret(&chunk));
}

//Stack point
struct Vm {
    pc: usize,
    stack: ArrayVec<f64, 256>,
}


#[derive(Default, Debug)]
struct Chunk {
    //Wait shit, in the lexer these are pushed as Tokens
    //but in the disassembler they're interpreted as Op's
    //figure out wtf is going on there
    code: Vec<u8>,
    const_pool: Vec<f64>,
    lines: Vec<u32>,
}

impl Vm {
    fn interpret(&mut self, chunk: &Chunk) -> Result<(), VmError> {

        while self.pc < chunk.code.len() {
            let instr = Op::from_repr(chunk.code[self.pc]).unwrap();

            match instr {
                OpConstant => {
                    self.pc += 1;
                    self.stack.push(chunk.const_pool[chunk.code[self.pc] as usize]);
                }

                OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                },

                OpNegate => {
                    let val = self.stack.last_mut().unwrap();
                    *val = -*val;
                },

                OpAdd => {
                    binary_op!(self.stack, +);
                },

                OpSubtract => {
                    binary_op!(self.stack, -);
                },

                OpMultiply => {
                    binary_op!(self.stack, *);
                },

                OpDivide => {
                    binary_op!(self.stack, /);
                },

                //_ => {}
            }

            self.pc += 1;
        }

        Ok(())
    }

    fn new() -> Vm {
        Vm {
            pc: 0,
            stack: ArrayVec::new(),
        }
    }
}


//"Disassembler"
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut i = 0;

        while i < self.code.len() {
            //fix this bullshit later
            let opcode = Op::from_repr(self.code[i]).unwrap();

            write!(f, "{:04} {:?}", i, opcode);

            match opcode {
                OpConstant => {
                    i += 1;
                    write!(f, "    {} '{}'", 
                        self.code[i], 
                        self.const_pool[self.code[i] as usize]);
                        //i += 1;
                }

                _ => {}
            }

            writeln!(f);

            i += 1;
        }

        Ok(())
    }
}

impl Chunk {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            const_pool: Vec::new(),
            lines: Vec::new(),
        }
    }

}

fn lex(source: &str) -> Result<Vec<Token>, VmError> {
    let mut line_num = 0;
    let mut iter = source.chars().peekable();

    let mut tokens = Vec::new();
    let mut curr_idx: usize = 0;

    while let Some(c) = iter.next() {

        if c.is_whitespace() {
            if c == '\n' { line_num += 1; }
            curr_idx += 1;
            continue;
        }

        assert_eq!(source.as_bytes()[curr_idx] as char, c);

        //Handle compile time errors later
        //who needs error handling anyways
        let start_idx = curr_idx;

        let token_type = match c {

            '>' => match iter.peek() {
                Some('=') => { iter.next(); curr_idx += 1; GreaterEqual }
                _ => Greater
            }

            '<' => match iter.peek() {
                Some('=') => { iter.next(); curr_idx += 1; LessEqual }
                _ => Less
            }

            '=' => match iter.peek() {
                Some('=') => { iter.next(); curr_idx += 1; EqualEqual }
                _ => Equal
            }


            '!' => match iter.peek() {
                Some('=') => { iter.next(); curr_idx += 1; BangEqual }
                _ => Bang
            }

            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            
            '/' => match iter.peek() {
                Some(&'/') => { 
                    while iter.next() != Some('\n') {};
                    line_num += 1;
                    continue;
                }
                
                _ => Slash
            }

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
                while iter.next() != Some('\"') {
                    curr_idx += 1;            
                }

                curr_idx += 1;

                //Do nothing with it for now

                if iter.peek().is_none() {
                    println!("{}", *iter.peek().unwrap());
                    panic!("Hahaha sucker, not gonna tell you what
                    the error here is, fuck you and good luck debugging lmao");
                }

                Str
            }

            //Implement later
            c if c.is_ascii_digit() => {

                //Meh whatever
                while let Some(digit) = iter.peek().unwrap().to_digit(10) {
                    iter.next();
                    curr_idx += 1;
                }

                if iter.peek() == Some(&'.') {
                    iter.next();
                    curr_idx += 1;
                }

                while let Some(digit) = iter.peek().unwrap().to_digit(10) {
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
                    _ => Identifier
                }
            }

            _ => panic!()
        };

        curr_idx += 1;

        let mut token = Token {
            kind: token_type,
            line_num: line_num,
            content: &source[start_idx..curr_idx]
        };


        println!("{}\t{:?}", token.content, token.kind);

        tokens.push(token);
    }


    //Must alter once you start reading into multiple chunks
    tokens.push(Token {
        kind: Eof,
        line_num: line_num,
        content: ""
    });

    Ok(tokens)
}

/*
fn compile(tokens: Vec<Token>) -> Result<Vec<Op>, VmError> {
    let mut opcodes = Vec::new();
    let mut const_pool = Vec::new();

    let mut prev_tok = Token {
        kind: None,
        line_num: 0,
        content: ""
    };

    for token in tokens {

        let opcode = match token.kind {
            Number => {
                const_pool.push(token.content.parse::<f64>());
                if const_pool.len() > 256 { panic!("Too many consts in const pool!"); }
                OpConstant
            },
            
            _ => todo!()
        };

    }

    Ok(opcodes)
}*/


#[derive(Debug, FromRepr)]
#[repr(u8)]
enum Op {
    OpConstant,
    OpReturn,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
}

#[derive(Debug)]
enum VmError {
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
struct Token<'a> {
    kind: TokenType,
    line_num: usize,
    //How to make this an iterator over
    content: &'a str
}

#[derive(Debug, FromRepr)]
enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character s.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals; Might change the implementation of these later
    //to utilize the way Clox stores literals
    Identifier, Str, Number,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    //None is strictly for 
    Newline, Eof, None
}


