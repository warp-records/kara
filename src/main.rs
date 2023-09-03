
use crate::Token::*;
use Op::*;
use strum_macros::FromRepr;
use std::fmt;
use std::fs;
use arrayvec::ArrayVec;
use std::env;

use peeking_take_while::PeekableExt;

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

        chunk.lex(&source);
    }

    println!("{:?}", vm.interpret(&chunk));
    println!("{}", chunk);
}

//Stack point
struct Vm {
    pc: usize,
    stack: ArrayVec<f64, 256>,
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



#[derive(Default, Debug)]
struct Chunk {
    code: Vec<u8>,
    const_pool: Vec<f64>,
    lines: Vec<u32>,
}

//"Disassembler"
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut i = 0;

        while i < self.code.len() {
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

    fn lex(&mut self, source: &str) -> Result<(), VmError> {
        let mut line_num = 0;
        let mut iter = source.chars().peekable();

        while let Some(c) = iter.next() {
            if c == '\n' {
                line_num += 1;
            }

            if c.is_whitespace() {
                continue;
            }

            //Handle compile time errors later
            //who needs error handling anyways
            let token = match c {

                '>' => match iter.peek() {
                    Some('=') => { iter.next(); GreaterEqual }
                    _ => Greater
                }

                '<' => match iter.peek() {
                    Some('=') => { iter.next(); LessEqual }
                    _ => Less
                }

                '=' => match iter.peek() {
                    Some('=') => { iter.next(); EqualEqual }
                    _ => Equal
                }


                '!' => match iter.peek() {
                    Some('=') => { iter.next(); BangEqual }
                    _ => Bang
                }

                '+' => Plus,
                '-' => Minus,
                '*' => Star,
                
                '/' => match iter.peek() {
                    Some(&'/') => { 
                        iter.by_ref().skip_while(|c| *c != '\n');
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
                    let literal: Vec<_> = iter.by_ref().peeking_take_while(|c| *c != '"').collect();

                    //Do nothing with it for now

                    if iter.peek() != Some(&'\"') {
                        panic!("Hahaha sucker good luck debugging lmfao");
                    }

                    Str
                }

                //Implement later
                c if c.is_ascii_digit() => {

                    //Meh whatever
                    while let Some(digit) = iter.peek().unwrap().to_digit(10) {
                        iter.next();
                    }

                    if iter.peek() == Some(&'.') {
                        iter.next();
                    }

                    while let Some(digit) = iter.peek().unwrap().to_digit(10) {
                        iter.next();
                    }

                    Number
                }


                c if c.is_alphabetic() || c == '_' => {
                    let mut lexeme = String::new();

                    while matches!(iter.peek(), Some(c) if c.is_alphanumeric()) || c == '_' {
                        lexeme.push(c);
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

            self.code.push(token as u8);
            self.lines.push(line_num);
        }
        
        //Must alter once you start reading into multiple chunks
        self.code.push(Eof as u8);
        Ok(())
    }
}


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



enum Token {
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

    Error, Eof
}


