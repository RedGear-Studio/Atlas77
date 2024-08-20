use atlas_core::prelude::*;

lexer_builder!();
//number!(enable_f64: false, enable_i64: true);
symbols!(
    '$' => DollarSign,
    '#' => HashTag,
    '.' => Dot,
    ':' => Colon,
    ';' => SemiColon,
    '&' => Ampersand,
    '[' => LBracket,
    ']' => RBracket
);
keywords!(
    "section", "code", "start", "push", "call", "print", "hlt", "dup", "lt", "lte", "gt", "gte",
    "jmp", "jmpz", "jmpnz", "ret", "swap", "add", "sub", "mul", "div"
);

pub fn identifier_system(c: char, state: &mut LexerState) -> Option<Token> {
    if c.is_alphabetic() || c == '_' {
        let start = state.current_pos;
        let mut s = String::new();
        s.push(c);
        state.next();
        while let Some(c) = state.peek() {
            if c.is_alphabetic() || *c == '_' {
                s.push(*c);
                state.next();
            } else {
                break;
            }
        }
        return Some(Token::new(
            Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            TokenKind::Literal(Literal::Identifier(Intern::new(s))),
        ));
    }
    None
}
