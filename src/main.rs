


use std::env;
use std::fs;

pub mod vm;
pub mod lex;

use vm::*;
use lex::*;

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

