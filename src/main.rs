
use strum_macros::FromRepr;
use std::fmt;
use arrayvec::ArrayVec;


macro_rules! binary_op {
    ($stack:expr, $op:tt) => {{
        let b = $stack.pop().unwrap();
        let a = $stack.pop().unwrap();
        $stack.push(a $op b);
    }};
}

fn main() {
    use Op::*;

    let mut chunk = Chunk::default();

    chunk.code.push(OpConstant as u8);
    chunk.code.push(0);
    chunk.const_pool.push(1.2);
    chunk.lines.push(123);

    chunk.code.push(OpConstant as u8);
    chunk.code.push(1);
    chunk.const_pool.push(3.4);
    chunk.lines.push(123);

    chunk.code.push(OpAdd as u8);
    chunk.lines.push(123);

    chunk.code.push(OpConstant as u8);
    chunk.code.push(2);
    chunk.const_pool.push(5.6);
    chunk.lines.push(123);

    chunk.code.push(OpDivide as u8);
    chunk.lines.push(123);

    chunk.code.push(OpNegate as u8);
    chunk.lines.push(123);

    chunk.code.push(OpReturn as u8);
    chunk.lines.push(123);

    let mut vm = Vm::new();

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
