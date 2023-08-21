use std::fmt::{Display, Error};
use colored::Colorize;
use crate::{span::Span, file::FilePath};

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    span: Span,
    severity: Severity,
    code: u32,
    message: String,
    path: FilePath,
    context: String,
}

impl Report {
    pub fn new(span: Span, severity: Severity, code: u32, message: String, path: FilePath, context: String) -> Self {
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

#[derive(Debug, Clone, PartialEq)]
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

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res_code = self.path.clone().get_code();
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

    
    {}",self.severity, self.code, self.message, FilePath::get_file_name(&self.path.path),
                        line.to_string().blue(), column.to_string().blue(), line.to_string().blue(), txt, self.context)
                }
                Severity::Warning => todo!(),
                Severity::Note => todo!(),
                Severity::Tip => todo!(),
            };
        };
        Ok(())
    }
}