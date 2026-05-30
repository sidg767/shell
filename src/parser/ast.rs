#[derive(Debug, Clone, PartialEq)]
pub enum Redirect {
    In(String),
    Out(String),
    Append(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirects: Vec<Redirect>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
    And,  // &&
    Or,   // ||
    Semi, // ;
}

#[derive(Debug, Clone, PartialEq)]
pub struct AndOrList {
    pub pipelines: Vec<(Pipeline, Option<LogicOp>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    pub list: AndOrList,
}
