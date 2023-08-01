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
    program: (statement NEWLINE?)*;
    statement: (let_statement | return_statement | expression_statement | if_statement | block | function_declaration);
    let_statement: LET IDENTIFIER (COLON type)? (EQ expression)? SEMICOLON;
    return_statement: RETURN expression SEMICOLON;
    expression_statement: expression SEMICOLON;
    if_statement: IF LPAREN expression RPAREN statement (ELSE statement)?;
    block: LBRACE NEWLINE* (statement NEWLINE?)* RBRACE;
    function_declaration: FN IDENTIFIER LPAREN (parameter (COMMA parameter)*)? RPAREN (ARROW type)? block;
    parameter: IDENTIFIER COLON type;
    type: I32 | F32 | BOOL | VOID | CHAR;
    expression: (assignment | logical_or);
    assignment: IDENTIFIER EQ assignment | logical_or;
    logical_or: logical_and (OR logical_and)*;
    logical_and: equality (AND equality)*;
    equality: comparison ((EQ | NEQ) comparison)*;
    comparison: addition ((LT | GT | LTE | GTE) addition)*;
    addition: multiplication ((PLUS | MINUS) multiplication)*;
    multiplication: unary ((MULT | DIV | MOD) unary)*;
    unary: (NOT | MINUS) unary | call;
    call: primary (LPAREN (expression (COMMA expression)*)? RPAREN)*;
    primary: (TRUE | FALSE | NUMBER | STRING | IDENTIFIER | LPAREN expression RPAREN);
    NUMBER: [0-9]+;
    STRING: '"' .*? '"';