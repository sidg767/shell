use crate::lexer::tokens::Token;
use crate::parser::ast::{AndOrList, AstNode, Command, LogicOp, Pipeline, Redirect};
use crate::error::shell_error::ShellError;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    pub fn parse(&mut self) -> Result<AstNode, ShellError> {
        let list = self.parse_and_or_list()?;
        Ok(AstNode { list })
    }

    fn parse_and_or_list(&mut self) -> Result<AndOrList, ShellError> {
        let mut pipelines = Vec::new();

        if self.peek().is_none() {
            return Ok(AndOrList { pipelines });
        }

        loop {
            // Skip empty statements like `;;` or starting with `;`
            while let Some(Token::Semi) = self.peek() {
                self.advance();
            }
            if self.peek().is_none() {
                break;
            }

            let pipeline = self.parse_pipeline()?;

            match self.peek() {
                Some(Token::AndIf) => {
                    self.advance();
                    pipelines.push((pipeline, Some(LogicOp::And)));
                }
                Some(Token::OrIf) => {
                    self.advance();
                    pipelines.push((pipeline, Some(LogicOp::Or)));
                }
                Some(Token::Semi) => {
                    self.advance();
                    pipelines.push((pipeline, Some(LogicOp::Semi)));
                }
                _ => {
                    pipelines.push((pipeline, None));
                    break;
                }
            }
        }

        Ok(AndOrList { pipelines })
    }

    fn parse_pipeline(&mut self) -> Result<Pipeline, ShellError> {
        let mut commands = Vec::new();

        loop {
            let command = self.parse_command()?;
            commands.push(command);

            match self.peek() {
                Some(Token::Pipe) => {
                    self.advance();
                }
                _ => break,
            }
        }

        Ok(Pipeline { commands })
    }

    fn parse_command(&mut self) -> Result<Command, ShellError> {
        let mut name = String::new();
        let mut args = Vec::new();
        let mut redirects = Vec::new();
        let mut name_set = false;

        loop {
            match self.peek() {
                Some(Token::Word(w)) => {
                    if !name_set {
                        name = w.clone();
                        name_set = true;
                    } else {
                        args.push(w.clone());
                    }
                    self.advance();
                }
                Some(Token::RedirectIn)
                | Some(Token::RedirectOut)
                | Some(Token::RedirectAppend) => {
                    let op = self.advance().cloned().ok_or(ShellError::UnexpectedEof { context: "redirect" })?;
                    if let Some(Token::Word(file)) = self.peek() {
                        let file = file.clone();
                        self.advance();
                        match op {
                            Token::RedirectIn => redirects.push(Redirect::In(file)),
                            Token::RedirectOut => redirects.push(Redirect::Out(file)),
                            Token::RedirectAppend => redirects.push(Redirect::Append(file)),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err(ShellError::SyntaxError { token: format!("{:?}", op) });
                    }
                }
                _ => break,
            }
        }

        if !name_set && redirects.is_empty() {
            return Err(ShellError::EmptyCommand);
        }

        Ok(Command {
            name,
            args,
            redirects,
        })
    }
}
