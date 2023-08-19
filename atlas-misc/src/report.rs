use crate::span::Span;

#[derive(Debug)]
pub struct Report {
    pub span: Span,
    pub severity: Severity,
    pub code: u32,
    pub message: String,
    pub line: u32,  //From span
    pub column: u32,//From span
}

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Tip,    
}