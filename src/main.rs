mod lexer;
mod parser;
use std::io::{self, Write};
use console::style;

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
                Ok(val) =>  println!("{}", style(val).green()),
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
        println!("Functions list - ['abs(x)', 'sqrt(x)', 'sin(x)', 'cos(x)', 'tan(x)', 'ln(x)', 'deg(rad)', 'pow(b, e)', 'log(b, x)']");
        println!("Constant list: ['PI', 'E', 'PHI']");
        println!("Binary operation list - ['<', '<=', '==', '!=', '>=', '>', '+', '-', '*', '/', '%']");
        println!("Unary operation list - ['+', '-', '!']");
        println!("Strings can be used with '' or \"\" and are converted to their length");
        println!("Argv (Optional): [program_name] ... -- Note: This is unreliable because of the terminal inner parser");
        println!("Enter 'clear' or 'cls' to clear screen");
        println!("Enter 'exit' to exit");
        println!();
        loop { 
            let input = get_input("Enter expression: ");
            if input == "exit" { return }
            else if ["clear", "cls"].contains(&input.as_str()) { console::Term::stdout().clear_screen().unwrap() }
            else { execute(&input); } 
        }
    }
}