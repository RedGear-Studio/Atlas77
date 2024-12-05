use atlas_core::prelude::*;

lexer_builder! {
    DefaultSystem {
        number: true,
        symbol: true,
        keyword: true,
        string: true,
        whitespace: {
            allow_them: false,
            use_system: true,
        },
    },
    Symbols {
        Single {
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            ',' => Comma,
            '+' => OpAdd,
            '/' => OpDiv,
            '*' => OpMul,
            '^' => OpPow,
            '%' => OpMod,
            '\\' => BackSlash,
            '_' => Underscore,
            ';' => Semicolon,
            '\'' => Quote,
            '?' => Interrogation,
        },
        Either {
            '=' => '=' => OpEq, OpAssign,
            '!' => '=' => OpNEq, Bang,
            '.' => '.' => DoubleDot, Dot,
            ':' => ':' => DoubleColon, Colon,
            '-' => '>' => RArrow, OpSub,
            '<' => '=' => OpLessThanEq, OpLessThan,
            '>' => '=' => OpGreaterThanEq, OpGreaterThan,
            '&' => '&' => OpAnd, Ampersand,
            '|' => '|' => OpOr, Pipe,
            '~' => '>' => FatArrow, Tilde,
        }
    },
    Keyword {
        "function", "if", "else", "class", "struct", "true", "false", "let", "public", "private", "extends", "interface", "import", "return", "enum", "List", "then", "end", "do",
        "self", "Self", "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64", "string", "List", "char",
    },
    Number {
        trailing {
            "_i8"   => i8   => I8,
            "_i16"  => i16  => I16,
            "_32"   => i32  => I32,
            "_i64"  => i64  => I64,
            "_i128" => i128 => I128,
            "_u8"   => u8   => U8,
            "_u16"  => u16  => U16,
            "_u32"  => u32  => U32,
            "_u64"  => u64  => U64,
            "_u128" => u128 => U128,
            "_f32"  => f32  => F32,
            "_f64"  => f64  => F64
        },
        float: true,
        u_int: true,
        int: true
    },
}
