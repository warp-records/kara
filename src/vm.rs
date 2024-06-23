
use std::fmt;
use strum_macros::FromRepr;
use arrayvec::ArrayVec;
use Op::*;

macro_rules! binary_op {
    ($stack:expr, $op:tt) => {{
        let b = $stack.pop().unwrap();
        let a = $stack.pop().unwrap();
        $stack.push(a $op b);
    }};
}

//Stack point
pub struct Vm {
    pc: usize,
    stack: ArrayVec<f64, 256>,
}

#[derive(Debug)]
pub enum VmError {
    CompileError,
    RuntimeError,
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

impl Op {

    pub fn to_repr(&self) -> u8 {
        *self as u8
    }
}


#[derive(Default, Debug)]
pub struct Chunk {
    //Wait shit, in the lexer these are pushed as Tokens
    //but in the disassembler they're interpreted as Op's
    //figure out wtf is going on there
    code: Vec<u8>,
    const_pool: Vec<f64>,
    lines: Vec<u32>,
}

impl Vm {
    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), VmError> {

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
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            const_pool: Vec::new(),
            lines: Vec::new(),
        }
    }

}
