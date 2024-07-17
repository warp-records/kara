use crate::Op::*;
use crate::Token;
use crate::TokenType;
use crate::Value;
use std::vec::IntoIter;
use strum_macros::FromRepr;
use Precedence::*;
use TokenType::*;

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
                    content: "",
                },

                current: Token {
                    kind: Blank,
                    line_num: 0,
                    content: "",
                },
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
    pub fn compile(&mut self) {
        //-> Result<Vec<u8>, VmError> {
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

        if self.parser.current.kind != RightParen {
            panic!("Expected ')'");
        }
        self.advance();
    }

    fn literal(&mut self) {
        let op = match self.parser.previous.kind {
            True => OpTrue,
            False => OpFalse,
            Nil => OpNil,
            _ => unreachable!(),
        };

        self.bytecode.push(op as u8);
    }

    fn string(&mut self) {
        let string = self.parser.previous.content.to_owned();
        self.const_pool.push(Value::Str(string));
        self.bytecode.push(OpConstant as u8);
        self.bytecode.push((self.const_pool.len() - 1) as u8);
    }

    fn number(&mut self) {
        let val = self.parser.previous.content.parse::<f64>().unwrap();
        self.const_pool.push(Value::Number(val));
        if self.const_pool.len() > 256 {
            panic!("No room in const pool");
        }

        self.bytecode.push(OpConstant as u8);
        self.bytecode.push((self.const_pool.len() - 1) as u8);
    }

    //keep for now, possibly remove later
    fn unary(&mut self) {
        let op_type = self.parser.previous.kind;
        self.parse_precedence(Unary);

        match op_type {
            Minus => {
                self.bytecode.push(OpNegate as u8);
            },
            Bang => {
                self.bytecode.push(OpNot as u8);
            }, 
            _ => unreachable!(),
        };
    }

    fn binary(&mut self) {
        let op_type = self.parser.previous.kind;
        let parse_rule = self.get_rule(op_type);
        self.parse_precedence(Precedence::from_repr(parse_rule.prec as u8 + 1).unwrap());

        let op = match op_type {
            Plus => OpAdd,
            Minus => OpSubtract,
            Star => OpMultiply,
            Slash => OpDivide,
            //BE, LE, and GE are inverted later
            BangEqual | EqualEqual => OpEqual,
            Greater | LessEqual => OpGreater,
            Less | GreaterEqual => OpLess,
            _ => unreachable!(),
        };

        self.bytecode.push(op as u8);

        match op_type {
            BangEqual | GreaterEqual | LessEqual => self.bytecode.push(OpNot as u8),
            _ => {}
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
            LeftParen => ParseRule {
                prefix: |s| s.grouping(),
                infix: |_s| {},
                prec: Null,
            },
            Minus => ParseRule {
                prefix: |s| s.unary(),
                infix: |s| s.binary(),
                prec: Term,
            },
            Plus => ParseRule {
                prefix: |_s| {},
                infix: |s| s.binary(),
                prec: Term,
            },
            Slash => ParseRule {
                prefix: |_s| {},
                infix: |s| s.binary(),
                prec: Factor,
            },
            Star => ParseRule {
                prefix: |_s| {},
                infix: |s| s.binary(),
                prec: Factor,
            },
            Number => ParseRule {
                prefix: |s| s.number(),
                infix: |_s| {},
                prec: Null,
            },
            True | False | Nil => ParseRule {
                prefix: |s| s.literal(),
                infix: |_s| {},
                prec: Null,
            },
            Bang => ParseRule {
                prefix: |s| s.unary(),
                infix: |_s| {},
                prec: Null,
            },
            BangEqual | EqualEqual => ParseRule {
                prefix: |_s| {},
                infix: |s| s.binary(),
                prec: Equality,
            },
            Greater | GreaterEqual | Less | LessEqual => ParseRule {
                prefix: |_s| {},
                infix: |s| s.binary(),
                prec: Comparison,
            },
            TokenType::Str => ParseRule {
                prefix: |_s| {},
                infix: |s| s.string(),
                prec: Null,
            },

            _ => ParseRule {
                prefix: |_s| {},
                infix: |_s| {},
                prec: Null,
            },
        }
    }
}
