
use crate::Token;
use crate::vm::VmError::*;

struct Parser {
    previous: Token,
    current: Token,
}

pub struct Compiler {
    parser: Parser,
    tokens: Vec<Token>,
}

impl Compiler {

    fn expression() {
        grouping();
        
    }

    fn grouping() {

    }

    pub fn new(tokens: Vec<Token>) {
        self.tokens = tokens.into_iter();
        Parser {
            previous: Token {
                kind: None,
                line_num: 0,
                content: ""
            },

            current: {
                Token
                kind: None,
                line_num: 0,
                content: ""
            }
        }
    }

    pub fn compile() -> Result<Vec<u8>, VmError> {

        let mut opcodes = vec![];
        let mut tokens_iter = self.tokens.iter();
        
        for token in tokens_iter {
            parser.previous = parser.current;
            parser.current = token;

            let opcode = match token.kind {
                Number => {
                    const_pool.push(token.content.parse::<f64>());
                    /*panic!("Too many consts in const pool!");*/ 
                    if const_pool.len() > 256 { return Err(CompileError); }
                    OpConstant
                },

                //LeftParen => {
                //    expression();
                //    if (tokens_iter.next() != Some(")")) { return Err(CompileError); }
                //}

                _ => todo!()
            };

            opcodes.push(opcode as u8);
            if opcode == OpConstant { opcodes.push(const_pool.len()-1); }
        }

        Ok(opcodes)
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


*/
