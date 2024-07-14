
use crate::Token;
use crate::TokenType;
use TokenType::*;
use crate::Value;
use crate::vm::VmError;
use crate::Op::*;
use Precedence::*;
use std::vec::IntoIter;
use strum_macros::FromRepr;

//only needs lifetime parameter because
//token contains string slice
struct Parser<'a> {
    previous: Token<'a>,
    current: Token<'a>,
}

struct ParseRule {
    prefix: fn(&mut Compiler),
    infix: fn(&mut Compiler),
    prec: Precedence,
}

//const null_fn = |s: &mut Compiler| {};

//impl ParseRule {
//    fn new(pre: fn(&mut self), in: fn(&mut self), prec: Precedence) -> Self {
//        ParseRule{ prefix: pre, infix: in, prec: prec }
//    }
//}

#[derive(FromRepr, PartialEq, PartialOrd)]
#[repr(u8)]
enum Precedence {
    Null,
    Assignemnt,
    Or,
    And,
    Equality,
    Comparison, // > < >= <=
    Term,       // + -
    Factor,     // * /
    Unary,      // !
    Call,       // . ()
    Primary,
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    tokens: IntoIter<Token<'a>>,
    //leave as public for now, but possibly change later
    pub bytecode: Vec<u8>,
    pub const_pool: Vec<Value>,
}

impl<'a> Compiler<'a> {

    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens.into_iter(),
            parser: Parser {
                previous: Token {
                    kind: Blank,
                    line_num: 0,
                    content: ""
                },

                current: Token {
                    kind: Blank,
                    line_num: 0,
                    content: ""
                }
            },
            const_pool: vec![],
            bytecode: vec![],
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;
        self.parser.current = self.tokens.next().unwrap();
    }

    //just implement the authors way, and change later
    pub fn compile(&mut self) {//-> Result<Vec<u8>, VmError> {
        self.advance();
        self.expression();

        //if self.parser.current.kind != Eof { panic!("Expected ')'"); }


        //Ok(std::mem::take(&mut self.bytecode))
    }

    fn expression(&mut self) {
        self.parse_precedence(Assignemnt);
    }

    //prolly gonna have to change this later
    fn grouping(&mut self) {
        //Never be afraid to express yourself :)
        self.expression();
        //which one ?

        if self.parser.current.kind != RightParen { panic!("Expected ')'"); }
        self.advance();
    }

    fn number(&mut self) {
        let val = self.parser.previous.content.parse::<f64>().unwrap();
        self.const_pool.push(Value::Number(val));
        if self.const_pool.len() > 256 { panic!("No room in const pool"); }

        self.bytecode.push(OpConstant as u8);
        self.bytecode.push((self.const_pool.len()-1) as u8);
    }

    //keep for now, possibly remove later
    fn unary(&mut self) {
        let op_type = self.parser.previous.kind;
        self.parse_precedence(Unary);

        match op_type {
            Minus => {
                self.bytecode.push(OpNegate as u8);
            },
            _ => unreachable!(),
        };
    }

    fn binary(&mut self) {
        let op_type = self.parser.previous.kind;
        let parse_rule = self.get_rule(op_type);
        self.parse_precedence(Precedence::from_repr(parse_rule.prec as u8 + 1).unwrap());

        match op_type {
            Plus => self.bytecode.push(OpAdd as u8),
            Minus => self.bytecode.push(OpSubtract as u8),
            Star => self.bytecode.push(OpMultiply as u8),
            Slash => self.bytecode.push(OpDivide as u8),
            _ => unreachable!(),
        };
    }

    //What the fuck
    fn parse_precedence(&mut self, prec_level: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.parser.previous.kind).prefix;
        
        //for some reason it won't let me cast a
        //closure that's a local variable
        if prefix_rule as usize == self.get_rule(Blank).prefix as usize {
            panic!("Expected expression");
        }

        prefix_rule(self);

        while prec_level <= self.get_rule(self.parser.current.kind).prec {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous.kind).infix;
            infix_rule(self);
        }
    }

    fn get_rule(&mut self, token_type: TokenType) -> ParseRule {
        match token_type {
            LeftParen => ParseRule{ prefix: |s| s.grouping(), infix: |s| {}, prec: Null },
            Minus =>     ParseRule{ prefix: |s| s.unary(), infix: |s| s.binary(), prec: Term },
            Plus =>      ParseRule{ prefix: |s| {}, infix: |s| s.binary(), prec: Term },
            Slash =>     ParseRule{ prefix: |s| {}, infix: |s| s.binary(), prec: Factor },
            Star =>      ParseRule{ prefix: |s| {}, infix: |s| s.binary(), prec: Factor },
            Number =>    ParseRule{ prefix: |s| s.number(), infix: |s| {}, prec: Null },
            
            _ =>         ParseRule{ prefix: |s| {}, infix: |s| {}, prec: Null },
        }
    }

}

