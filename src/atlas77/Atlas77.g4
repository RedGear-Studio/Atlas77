grammar Atlas77;

//Lexer rules
    //Literals
    COMMA: ',';
    SEMICOLON: ';';
    COLON: ':';
    LPAREN: '(';
    RPAREN: ')';
    LBRACE: '{';
    RBRACE: '}';
    LBRACKET: '[';
    RBRACKET: ']';
    PLUS: '+';
    MINUS: '-';
    MULT: '*';
    DIV: '/';
    MOD: '%';
    EQ: '==';
    NEQ: '!=';
    LT: '<';
    GT: '>';
    LTE: '<=';
    GTE: '>=';
    ARROW: '->';
    NOT: '!';
    OR: '||';
    AND: '&&';
    WS: [ \t]+ -> skip;
    NEWLINE: '\r'? '\n';

    //Keywords
    TRUE: 'true';
    FALSE: 'false';
    LET: 'let';
    CONST: 'const';
    RETURN: 'return';
    AS: 'as';
    IF: 'if';
    ELSE: 'else';
    FN: 'fn';
    I32: 'i32';
    F32: 'f32';
    BOOL: 'bool';
    VOID: 'void';
    CHAR: 'char';
    IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;

// Parser rules
    program: file?;
    file: (function_declaration | constant_declaration)* EOF;
    constant_declaration: CONST IDENTIFIER COLON type EQ expr SEMICOLON;
    statement: (let_statement | return_statement | expr_statement | if_statement | block | function_declaration);
    let_statement: LET IDENTIFIER COLON type (EQ expr)? SEMICOLON;
    return_statement: RETURN expr SEMICOLON;
    expr_statement: expr SEMICOLON;
    if_statement: IF LPAREN expr RPAREN statement (ELSE statement)?;
    block: LBRACE NEWLINE* (statement NEWLINE?)* RBRACE;
    function_declaration: FN IDENTIFIER parameters? (ARROW type)? block;
    parameters: LPAREN parameter (COMMA parameter)* RPAREN;
    parameter: IDENTIFIER COLON type;
    type: I32 | F32 | BOOL | VOID | CHAR;
    expr
        : expr (MULT | DIV | MOD) expr
        | expr (PLUS | MINUS) expr
        | expr (EQ | NEQ | LT | LTE | GT | GTE) expr
        | expr (OR | AND) expr
        | atom;
    
    atom: NUMBER | LPAREN expr RPAREN | TRUE | FALSE | call;
    call: IDENTIFIER LPAREN (expr (COMMA expr)*)? RPAREN;
    NUMBER: [0-9]+;
    STRING: '"' .*? '"';