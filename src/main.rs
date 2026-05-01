use shell::cli::repl;
fn main() -> Result<(),Box<dyn std::error::Error>>{ //Error's a trait and since size unknown, dyn used to make it a trait object
    repl::start()?;
    Ok(());
}
