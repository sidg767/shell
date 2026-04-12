use shell::cli::repl;
fn main() -> Result<(),Box<dyn std::error::Error>>{
    repl::start()?;
    Ok(());
}
