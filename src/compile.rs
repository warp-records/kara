
use crate::Token;
use crate::TokenType::*;
use crate::vm::VmError;
use crate::Op::*;
use ParsePrecedence::*;
use std::vec::IntoIter;

//only needs lifetime parameter because
//token contains string slice
struct Parser<'a> {
    previous: Token<'a>,
    current: Token<'a>,
}

enum ParsePrecedence {
    Null,
    Assignemnt,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    tokens: IntoIter<Token<'a>>,
    //machiiiine
    bytecode: Vec<u8>,
    const_pool: Vec<f64>,
    //prec_level: 
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

    //just implement the authors way, and change later
    pub fn compile(&mut self) -> Result<Vec<u8>, VmError> {
        
        while let Some(token) = self.tokens.next() {
            self.parser.previous = self.parser.current;
            self.parser.current = token;

            match token.kind {
                Number => {
                    self.number(token.content.parse::<f64>().unwrap());
                },

                LeftParen => {
                    //expression() is probably just compile()
                    self.grouping();
                },

                _ => todo!()
            };
        }

        //FIND way to clone without copying when you're not tired
        Ok(std::mem::take(&mut self.bytecode))
    }

    fn expression(&mut self) {

    }


    //prolly gonna have to change this later
    fn grouping(&mut self) -> Result<(), ()> {
        //Never be afraid to express yourself :)
        self.expression();
        if self.tokens.next().map(|token| token.kind) != Some(RightParen) { 
            Err(()) 
        } else {
            Ok(())
        }
    }

    fn number(&mut self, val: f64) -> Result<(), ()> {
        self.const_pool.push(val);
        if self.const_pool.len() > 256 { return Err(()); }

        self.bytecode.push(OpConstant as u8);
        self.bytecode.push((self.const_pool.len()-1) as u8);
        Ok(())
    }

    //keep for now, possibly remove later
    fn unary(&mut self) {
        let op = self.parser.previous.kind;
        self.expression();

        match op {
            Minus => {
                self.bytecode.push(OpNegate as u8);
            },
            _ => unreachable!(),
        };
    }

    fn parse_precedence(&mut self, level: ParsePrecedence) {

    }

}


/*
Lox book C code reference:

bool compile(const char* source, Chunk* chunk) {
    initScanner(source);
    advance();
    expression();
    consume(TOKEN_EOF, "Expect end of expression.");
}

//is this handled automatically in loop?
void initScanner(const char* source) {
    scanner.start = source;
    scanner.current = source;
    scanner.line = 1;
}


static void advance() {
    parser.previous = parser.current;
    parser.current = scanToken();
}


//scanner is for scanning TEXT, not tokens
//handle with for loop
Token scanToken() {
    scanner.start = scanner.current;

    if (isAtEnd()) return makeToken(TOKEN_EOF);

    return errorToken("Unexpected character.");
}

static void consume(TokenType type, const char* message) {
    if (parser.current.type == type) {
        advance();
        return;
    }

    errorAtCurrent(message);
}

static void number() {
    double value = strtod(parser.previous.start, NULL);
    emitConstant(value);
}

static void unary() {
    TokenType operatorType = parser.previous.type;

    // Compile the operand.
     expression();

     // Emit the operator instruction.
    switch (operatorType) {
        case TOKEN_MINUS: emitByte(OP_NEGATE); break;
        default: return; // Unreachable.
    }
}

static void parsePrecedence(Precedence precedence) {
  // What goes here?
}
*/
