

/*
fn compile(tokens: Vec<Token>) -> Result<Vec<Op>, VmError> {
    let mut opcodes = Vec::new();
    let mut const_pool = Vec::new();

    let mut prev_tok = Token {
        kind: None,
        line_num: 0,
        content: ""
    };

    for token in tokens {

        let opcode = match token.kind {
            Number => {
                const_pool.push(token.content.parse::<f64>());
                if const_pool.len() > 256 { panic!("Too many consts in const pool!"); }
                OpConstant
            },
            
            _ => todo!()
        };

    }

    Ok(opcodes)
}*/