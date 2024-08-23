use atlas_core::prelude::*;

lexer_builder!();
//number!(enable_f64: false, enable_i64: true);
symbols!(
    '$' => DollarSign,
    '#' => HashTag,
    '.' => Dot,
    ':' => Colon,
    '&' => Ampersand,
    '[' => LBracket,
    ']' => RBracket,
    '@' => AtSign
);
keywords!(
    "section",
    "start",
    "code",
    "push_i",
    "push_u",
    "push_f",
    "load_const",
    "pop",
    "add_i",
    "add_u",
    "add_f",
    "sub_i",
    "sub_u",
    "sub_f",
    "mul_i",
    "mul_u",
    "mul_f",
    "div_i",
    "div_u",
    "div_f",
    "dup",
    "swap",
    "rot",
    "jmp_nz",
    "jmp_z",
    "jmp",
    "extern_call",
    "call",
    "ret",
    "print_char",
    "print",
    "read",
    "read_i",
    "set_struct",
    "get_struct",
    "create_struct",
    "create_string",
    "str_len",
    "write_char",
    "read_char",
    "eq",
    "neq",
    "lt",
    "gt",
    "lte",
    "gte",
    "and",
    "or",
    "not",
    "hlt",
    "nop",
    "cast_to_int",
    "cast_to_uint",
    "cast_to_float",
    "cast_to_char",
    "cast_to_bool",
    "cast_to_ptr",
    "int",
    "u_int",
    "float",
    "char",
    "object",
    "string",
    "bool"
);

pub fn comment_system(c: char, state: &mut LexerState) -> Option<Token> {
    if c == ';' {
        let start = state.current_pos;
        state.next();
        while let Some(c) = state.peek() {
            if *c == '\n' {
                break;
            }
            state.next();
        }
        return Some(Token {
            span: Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            kind: TokenKind::WhiteSpace,
        });
    }

    None
}

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
