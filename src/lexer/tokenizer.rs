use crate::lexer::tokens::Token;

#[derive(Clone, Debug)]
enum State {
    Normal,
    SingleQuote,
    DoubleQuote,
    Escape(Box<State>),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut curr_token = String::new();
    let mut state = State::Normal;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match state {
            State::Normal => match c {
                ' ' | '\t' => {
                    if !curr_token.is_empty() {
                        tokens.push(Token::Word(curr_token.clone()));
                        curr_token.clear();
                    }
                }
                '\'' => state = State::SingleQuote,
                '\"' => state = State::DoubleQuote,
                '\\' => state = State::Escape(Box::new(State::Normal)),
                '|' => {
                    if !curr_token.is_empty() {
                        tokens.push(Token::Word(curr_token.clone()));
                        curr_token.clear();
                    }
                    if chars.peek() == Some(&'|') {
                        chars.next();
                        tokens.push(Token::OrIf);
                    } else {
                        tokens.push(Token::Pipe);
                    }
                }
                '&' => {
                    if !curr_token.is_empty() {
                        tokens.push(Token::Word(curr_token.clone()));
                        curr_token.clear();
                    }
                    if chars.peek() == Some(&'&') {
                        chars.next();
                        tokens.push(Token::AndIf);
                    } else {
                        tokens.push(Token::Ampersand);
                    }
                }
                '>' => {
                    if !curr_token.is_empty() {
                        tokens.push(Token::Word(curr_token.clone()));
                        curr_token.clear();
                    }
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(Token::RedirectAppend);
                    } else {
                        tokens.push(Token::RedirectOut);
                    }
                }
                '<' => {
                    if !curr_token.is_empty() {
                        tokens.push(Token::Word(curr_token.clone()));
                        curr_token.clear();
                    }
                    tokens.push(Token::RedirectIn);
                }
                ';' => {
                    if !curr_token.is_empty() {
                        tokens.push(Token::Word(curr_token.clone()));
                        curr_token.clear();
                    }
                    tokens.push(Token::Semi);
                }
                _ => curr_token.push(c),
            },
            State::SingleQuote => {
                if c == '\'' {
                    state = State::Normal;
                } else {
                    curr_token.push(c);
                }
            }
            State::DoubleQuote => {
                if c == '\"' {
                    state = State::Normal;
                } else if c == '\\' {
                    state = State::Escape(Box::new(State::DoubleQuote));
                } else {
                    curr_token.push(c);
                }
            }
            State::Escape(prev_state) => {
                curr_token.push(c);
                state = *prev_state;
            }
        }
    }

    if !curr_token.is_empty() {
        tokens.push(Token::Word(curr_token));
    }

    tokens
}
