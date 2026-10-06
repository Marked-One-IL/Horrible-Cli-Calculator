mod lexer;
mod parser;
use std::io::{self, Write};

fn get_input(msg: &'static str) -> String
{
    io::stdout().write_all(msg.as_bytes()).expect("Failed to write to stdout");
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input from stdin");
    input.trim_ascii().to_ascii_lowercase()
}
fn get_argv_input() -> Option<String>
{
    let mut args = std::env::args();
    let _ = args.next();
    let input = args.collect::<Vec<_>>().join(" ");
    (!input.is_empty()).then_some(input.to_ascii_lowercase())
}
fn execute(expr: &str) 
{
    match lexer::extract_tokens(expr) {
        Ok(tokens) => {
            match parser::solve(&tokens) {
                Ok(val) =>  println!("{}", val),
                Err(msg) => msg.print(expr, &tokens),
            }
        }
        Err(err) => {
            err.print(expr);
        }
    }
}

fn main() 
{
    if let Some(argv_input) = get_argv_input() { execute(&argv_input) }
    else 
    {
        println!("Functions list - sqrt, pow, log, sin, cos, tan");
        println!("Argv (Optional): [program_name] ... -- Note: This is unreliable on different terminals");
        println!("Enter 'exit' to exit");
        println!();
        loop { 
            let input = get_input("Enter expression: ");
            if input == "exit" { return }
            execute(&input); 
        }
    }
}