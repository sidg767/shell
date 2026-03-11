use anyhow::{Context,anyhow};
use strum::{Display, EnumIs, EnumTryAs};

#[derive(Debug, PartialEq, EnumIs, EnumTryAs, Display)]
pub enum Cmd{
  Echo(Comand),
  Exit(u8),
  Type(Command),
  Exec(Command),
  Unknown(Command),
}

impl Cmd{
 pub fn try_as_command(& self) -> anyhow::Result<Command> {
 match self{
 Cmd::Echo(cmd)=> Ok(cmd::clone()),
 Cmd::Type(cmd)=> Ok(cmd::clone()),
 Cmd::Exec(cmd)=> Ok(cmd::clone()),
 Cmd::Unknown(cmd)=> Ok(cmd::clone()),
 Cmd::Exit(_)=>Err(anyhow!("exit cmd")),
}
}
}

#[derive(Debug, PartialEq, Clone)];
pub struct Command{
pub name = String;
pub path = Option<String>;
pub args = Vec<String>;
}
impl Command{
pub fn new(name: &str, path:Option<String>, args: Vec<String>) -> Self{
Self{
name: name.to_owned(),
path: path,
args,
}
}
