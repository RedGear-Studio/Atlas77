use std::fmt::Display;
use colored::Colorize;
use crate::{span::Span, file::FilePath};

#[derive(Debug)]
pub struct Report<'a> {
    span: Span,
    severity: Severity,
    code: u32,
    message: String,
    path: FilePath<'a>,
    context: String,
}

impl<'a> Report<'a> {
    pub fn new(span: Span, severity: Severity, code: u32, message: String, path: FilePath<'a>, context: String) -> Self {
        Self {
            span,
            severity,
            code,
            message,
            path,
            context
        }
    }
}

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Tip,    
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "{}", "error".red().bold()),
            Severity::Warning => write!(f, "{}", "warning".yellow().bold()),
            Severity::Note => write!(f, "{}", "note".green().bold()),
            Severity::Tip => write!(f, "{}", "tip".blue().bold()),
        }
    }
}

impl<'a> Display for Report<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res_code = self.path.get_code();
        if let Ok(code) = res_code {
            let (line, column, code) = self.span.get_line_info(&code);
            let lines = code.lines();
            let mut txt: String = String::new();
            for l in lines {
                txt.push_str(&format!("\t|\t{}\n", l))
            }
            txt.push_str(&format!("\t|"));
            match self.severity {
                Severity::Error => {
                    write!(f, "{}[{}]: {}
\t--> {}:{}:{}  
\t|
{}{}

    
    {}",self.severity, self.code, self.message, self.path.file_name,
                        line.to_string().blue(), column.to_string().blue(), line.to_string().blue(), txt, self.context)?
                }
                _ => {

                }
            }
        }
        
        todo!()
    }
}