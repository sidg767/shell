pub mod cd;
pub mod echo;
pub mod pwd;
pub mod r#type;

pub fn handle(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "cd" => { cd::run(args); true }
        "echo" => { echo::run(args); true }
        "pwd" => { pwd::run(); true }
        "type" => { r#type::run(args); true }
        _ => false,
    }
}