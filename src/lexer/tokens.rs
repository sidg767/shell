#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,           // |
    RedirectIn,     // <
    RedirectOut,    // >
    RedirectAppend, // >>
    AndIf,          // &&
    OrIf,           // ||
    Semi,           // ;
    Ampersand,      // &
}
