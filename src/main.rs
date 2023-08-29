
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
    use Op::*;

    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let args: Vec<String> = env::args.collect();

    if args.len() == 2 {
        let source = fs::read_to_string(file_path)
            .expect("Error: unable to read file");

        scan_to_chunk(&source, &mut chunk);
    }

    println!("{:?}", vm.interpret(&chunk));
    println!("{}", chunk);
}

fn lex(source: &str, chunk: &mut Chunk) {
    let mut line_num = 0;
    let mut iter = source.chars().peekable();

    for c in iter {
        if c == '\n' {
            line_num += 1;
        }

        if c.is_whitespace() {
            continue;
        }

        let token = match c {

            '>' => match iter.peek() {
                '=' => { iter.next(); GreaterEqual }
                _ => Greater
            }

            '<' => match iter.peek() {
                '=' => { iter.next(); LessEqual }
                _ => Less
            }

            '=' => match iter.peek() {
                '=' => { iter.next(); EqualEqual }
                _ => Equal
            }


            '!' => match iter.peek() {
                '=' => { iter.next(); BangEqual }
                _ => Bang
            }

            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            
            '/' => match iter.peek() {
                '/' => { 
                    while (iter.next() != Some('\n'));
                    line_num++;
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
                let literal = iter.peeking_take_while(|c| c != '"').collect();

                if iter.peek() != Some("'") {
                    Identifier(literal)
                }
            }

            //Implement later
            c if c.is_ascii_digit() => {

                //Meh whatever
                while let Some(digit) = iter.peek().unwrap().to_digit(10) {
                    iter.next();
                }

                if iter.peek() == Some('.') {
                    iter.next();
                }

                while let Some(digit) = iter.peek().unwrap().to_digit(10) {
                    iter.next();
                }

                Number(0)
            }


            c if c.is_alphabetic() || c == '_' => {
                let mut lexeme = String::new();

                while (matches!(iter.peek(), Some(c) if c.is_alphanumeric()) || *c == '_') {
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
                    _ => Identifier(lexeme)
                }
            }
        }

        chunk.code.push(token);
        chunk.lines.push(line_num);
    }
    
    //Must alter once you start reading into multiple chunks
    chunk.push(Eof)
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
                Op::OpConstant => {
                    self.pc += 1;
                    self.stack.push(chunk.const_pool[chunk.code[self.pc] as usize]);
                }

                Op::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                },

                Op::OpNegate => {
                    let val = self.stack.last_mut().unwrap();
                    *val = -*val;
                },

                Op::OpAdd => {
                    binary_op!(self.stack, +);
                },

                Op::OpSubtract => {
                    binary_op!(self.stack, -);
                },

                Op::OpMultiply => {
                    binary_op!(self.stack, *);
                },

                Op::OpDivide => {
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

//"Dissasembler"
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut i = 0;

        while i < self.code.len() {
            let opcode = Op::from_repr(self.code[i]).unwrap();


            write!(f, "{:04} {:?}", i, opcode);

            match opcode {
                Op::OpConstant => {
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
    Identifier(String), String(String), Number(f64),
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, Eof
}
