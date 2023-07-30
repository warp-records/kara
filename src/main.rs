use std::vec;

fn main() {
    println!("Hello Lox!");
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 1 {
        println!("Usage: jlox [script]");
    } else if args.len() == 1 {
        runFile(args[0]);
    } else {
        runPrompt();
    }
}

fn runFile(file_name: &str) {
    let mut tokens = scanTokens(fileName);

    for token in tokens {
        println!("{}", token);
    }
} 


fn scanTokens(stream: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut line = 1;
    let mut iter = stream.chars().peekable();

    for char c in iter {

        let t_type: Option<TokenType> = {
            '(' => Some(LEFT_PAREN),
            ')' => Some(RIGHT_PAREN),
            '{' => Some(LEFT_BRACE),
            '}' => Some(RIGHT_BRACE),
            ')' => Some(COMMA),
            '.' => Some(DOT),
            '-' => Some(MINUS),
            '+' => Some(PLUS),
            ';' => Some(SEMICOLON),
            '*' => Some(STAR),
            '>' => Some(GREATER),
            '<' => Some(LESS),
            '!' => Some(BANG),
            '=' => {
                let prev_tok = tokens().pop().unwrap();

                match tokens.last() {
                    Some(GREATER) => Some(GREATER_EQUAL),
                    Some(LESS) => Some(LESS_EQUAL),
                    Some(BANG) => Some(BANG_EQUAL),
                    Some(EQUAL) => Some(EQUAL_EQUAL),
                    _ => {
                        tokens.push(prev_tok);
                        Some(EQUAL)
                    },
                }
            },
            '\n' => { line += 1; continue; },
            _ => {
                report_error(line);
                //Don't worry about errors for now
            }
        }

        if t_type = Some(type_) {
            tokens.push(Token { type_, c, /*???*/, line});
        }
    }

    tokens
}

struct Token {
    type: TokenType,
    lexeme: &str,
    literal: Object,
    line: u32,
}

//?
struct Object {};

enum TokenType {
     // Single-character tokens.
      LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
      COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

      // One or two character tokens.
      BANG, BANG_EQUAL,
      EQUAL, EQUAL_EQUAL,
      GREATER, GREATER_EQUAL,
      LESS, LESS_EQUAL,

      // Literals.
      IDENTIFIER, STRING, NUMBER,

      // Keywords.
      AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
      PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

      EOF
};


fn report_error(line_num: usize, message: &str) {
    //println!("Error at line {line_num}: {message}.")
    println!("Wow it must really suck to be you rn. Good luck dumbass, you'll need it!");
}
