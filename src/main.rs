
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

fn scan_to_chunk(source: &str, chunk: &mut Chunk) {
    let lines: Vec<&str> = source.lines().collect();

    let line_num = 1;

    for (line_num, content) in lines.enumerate() {
        let exprs: Vec<&str> = lines.split_whitespace().collect();

        for expr in exprs {
            let token = TokenType::from(expr);

            chunk.push(token);
        }
    }

    //Must alter once you start reading into multiple chunks
    chunk.push(TokenType::Eof)
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
    // Literals.
    Identifier(&str), String(&str), Number(f64),
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, Eof
}

//Thanks ChatGPT!
impl TryFrom<&str> for TokenType {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(TokenType::Plus),
            "-" => Ok(TokenType::Minus),
            "*" => Ok(TokenType::Star),
            "/" => Ok(TokenType::Slash),
            "(" => Ok(TokenType::LeftParen),
            ")" => Ok(TokenType::RightParen),
            "{" => Ok(TokenType::LeftBrace),
            "}" => Ok(TokenType::RightBrace),
            ")," => Ok(TokenType::Comma),
            "." => Ok(TokenType::Dot),
            ";" => Ok(TokenType::Semicolon),
            "!" => Ok(TokenType::Bang),
            "!=" => Ok(TokenType::BangEqual),
            "=" => Ok(TokenType::Equal),
            "==" => Ok(TokenType::EqualEqual),
            ">" => Ok(TokenType::Greater),
            ">=" => Ok(TokenType::GreaterEqual),
            "<" => Ok(TokenType::Less),
            "<=" => Ok(TokenType::LessEqual),
            "and" => Ok(TokenType::And),
            "class" => Ok(TokenType::Class),
            "else" => Ok(TokenType::Else),
            "false" => Ok(TokenType::False),
            "for" => Ok(TokenType::For),
            "fun" => Ok(TokenType::Fun),
            "if" => Ok(TokenType::If),
            "nil" => Ok(TokenType::Nil),
            "or" => Ok(TokenType::Or),
            "print" => Ok(TokenType::Print),
            "return" => Ok(TokenType::Return),
            "super" => Ok(TokenType::Super),
            "this" => Ok(TokenType::This),
            "true" => Ok(TokenType::True),
            "var" => Ok(TokenType::Var),
            "while" => Ok(TokenType::While),


            _ => {
                if let Some(num) = s.parse::<f64>() {
                    Ok(TokenType::Number(num))
                } else {
                    let chars = s.chars();

                    if chars.first() == Some('"') && chars.last() == Some('"') && s.len() > 2 {
                        let inner = &s[1..s.len() - 1];

                        if inner.chars().all(|c| c != '"') {
                            Ok(TokenType::String(inner))
                        }
                    } else if let Some(first_char) = s.chars().next() {
                        if first_char.is_alphanumeric() || first_char == '_' {
                            Ok(TokenType::Identifier(s))
                        } else {
                            Error()
                        }
                    }
                }
            }, // Handle unrecognized tokens here
        }
    }
}


