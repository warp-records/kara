

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

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 { panic!("Expected filename"); }

    let source = fs::read_to_string(&args[1])
        .expect("Error: unable to read file");

    let tokens = lex(&source).unwrap();
    let mut compiler = Compiler::new(tokens);
    compiler.compile();

    let mut chunk = Chunk {
        bytecode: compiler.bytecode.clone(),
        const_pool: compiler.const_pool.clone(),
        //lines: Vec::new(),
    };
    chunk.bytecode.push(Op::OpReturn as u8);

    println!("{}", chunk);

    let mut vm = Vm::new();
    vm.interpret(&chunk);
    //println!("{}", vm.interpret(&chunk));
}

