use atlas_syntax::token::Token;
use atlas_misc::span::WithSpan;

pub fn pretty_print_tokens(tokens: Vec<WithSpan<Token>>) {
    for tok in tokens {
        print!("[{}:{{{}|{}}}], ", tok.value, tok.span.start, tok.span.end);
    }
}