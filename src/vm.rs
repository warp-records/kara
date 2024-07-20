use arrayvec::ArrayVec;
use std::fmt;
use strum_macros::FromRepr;
use Op::*;
use Value::*;

macro_rules! binary_op {
    ($stack:expr, $op:tt, $return_type:ident) => {{
        let b = match $stack.pop().unwrap() {
            Value::Number(val) => val,
            _ => panic!("Invalid operands"),
        };
        let a = match $stack.pop().unwrap() {
            Value::Number(val) => val,
            _ => panic!("Invalid operands"),
        };
        $stack.push(Value::$return_type(a $op b));
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Str(String),
    //Obj(Object),
}

//CI implementation uses a separate Object type,
//revisit later
//pub enum Object {
//   Str(String),
//}

pub struct Vm {
    pub pc: usize,
    pub stack: ArrayVec<Value, 256>,
}

#[derive(Debug)]
pub enum VmError {
    CompileError,
    RuntimeError,
}

#[derive(Debug, FromRepr)]
#[repr(u8)]
pub enum Op {
    OpConstant,
    OpTrue,
    OpFalse,
    OpNil,
    OpReturn,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
}

/*
impl Op {

    pub fn to_repr(&self) -> u8 {
        *self as u8
    }
}*/

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number(inner) => write!(f, "{:.2}", inner),
            Str(inner) => write!(f, "{}", inner),
            Bool(inner) => write!(f, "{}", if *inner { "true" } else { "false" }),
            Nil => write!(f, "Nil"),
        }
    }
}

/*
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> {
        match self {
            if discriminant(self) == discriminant(Number) &&
                discriminant(other) == discriminant(Number) {
                return ;
            }



        }
    }
}*/

#[derive(Default, Debug)]
pub struct Chunk {
    //Wait shit, in the lexer these are pushed as Tokens
    //but in the disassembler they're interpreted as Op's
    //figure out wtf is going on there
    pub bytecode: Vec<u8>,
    pub const_pool: Vec<Value>,
    //this field isn't used?
    //lines: Vec<u32>,
}

impl Vm {
    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), VmError> {
        while self.pc < chunk.bytecode.len() {
            let instr = Op::from_repr(chunk.bytecode[self.pc]).unwrap();

            match instr {
                OpConstant => {
                    self.pc += 1;
                    self.stack
                        .push(chunk.const_pool[chunk.bytecode[self.pc] as usize].clone());
                }

                OpReturn => {
                    println!("{}", self.stack.last().unwrap());
                }

                OpNegate => {
                    //I think this'll work
                    if let Number(val) = self.stack.last().unwrap() {
                        *self.stack.last_mut().unwrap() = Number(-val);
                    } else {
                        panic!("Operand must be a number");
                    }
                }

                OpAdd => {
                    match (
                        self.stack.last().clone().unwrap(),
                        self.stack.get(self.stack.len().wrapping_sub(2)).clone().unwrap()) {

                        (Value::Str(_), Value::Str(_))
                        | (Value::Number(_), Value::Str(_))
                        | (Value::Str(_), Value::Number(_)) => {
                       
                            let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                            self.stack.push(Str(format!("{a}{b}")));
                        },

                        (Value::Number(_), Value::Number(_)) => {
                            binary_op!(self.stack, +, Number);
                        }

                        _ => {
                            panic!("Invalid operand type");
                        }
                    }
                }

                OpSubtract => {
                    binary_op!(self.stack, -, Number);
                }

                OpMultiply => {
                    binary_op!(self.stack, *, Number);
                }

                OpDivide => {
                    binary_op!(self.stack, /, Number);
                }

                OpTrue => {
                    self.stack.push(Bool(true));
                }

                OpFalse => {
                    self.stack.push(Bool(false));
                }

                OpNil => {
                    self.stack.push(Nil);
                }

                OpNot => {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(Bool(val == Bool(false) || val == Nil));
                }

                OpEqual => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(Bool(a == b));
                }

                OpGreater => {
                    binary_op!(self.stack, >, Bool);
                }

                OpLess => {
                    binary_op!(self.stack, <, Bool);
                } //_ => {}
            }

            self.pc += 1;
        }

        Ok(())
    }

    pub fn new() -> Vm {
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

        while i < self.bytecode.len() {
            //fix this bullshit later
            let opcode = Op::from_repr(self.bytecode[i]).unwrap();

            //rust throws a fit if I don't put a stupid underscore beforehand
            _ = write!(f, "{:04} {:?}", i, opcode);

            match opcode {
                OpConstant => {
                    i += 1;
                    _ = write!(
                        f,
                        "    {} '{}'",
                        self.bytecode[i], self.const_pool[self.bytecode[i] as usize]
                    );
                    //i += 1;
                }

                _ => {}
            }

            _ = writeln!(f);

            i += 1;
        }

        Ok(())
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            const_pool: Vec::new(),
            //lines: Vec::new(),
        }
    }

    /*
    pub fn new(bytecode: Vec<u8>, const_pool: Vec<f64>) -> Self {
        Self {
            bytecode: bytecode,
            const_pool: const_pool,
        }
    }*/
}

//make a CPU in verilog next
