

use strum_macros::FromRepr;

use std::env;
use std::fs;

pub mod vm;
pub mod lex;
pub mod compile;

use vm::*;
use lex::*;
use compile::*;

fn main() {

    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let source = fs::read_to_string(&args[1])
            .expect("Error: unable to read file");

        //println!("{}", source);

        let tokens = lex(&source).unwrap();
        //println!("{:?}", tokens);
        let mut compiler = Compiler::new(tokens);
        let bytecode = compiler.compile().unwrap();

        for byte in bytecode {
            println!("{:?}", Op::from_repr(byte));
        }
        //println!("{:?}", bytecode);
    }

    println!("{:?}", vm.interpret(&chunk));
}

