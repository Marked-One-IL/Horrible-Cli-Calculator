mod lexer;
mod parser;
use std::panic;
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
    (!input.is_empty()).then_some(input)
}

fn main() 
{
    panic::set_hook(Box::new(|_| {})); // Remove panic clutter.

    if let Some(argv_input) = get_argv_input()
    {
        let res = panic::catch_unwind(||
        {
            let tokens =  lexer::extract_tokens(argv_input.as_str());
            println!("{}", parser::solve(&tokens));
        });
        if let Err(panic) = res 
        {
            if let Some(e) = panic.downcast_ref::<&str>() { println!("{}", e) }
            else if let Some(e) = panic.downcast_ref::<String>() { println!("{}", e) }
            else { panic::resume_unwind(panic) }
        }
    }
    else
    {
        println!("Functions list - sqrt, pow, log, sin, cos, tan");
        println!("Argv (Optional): [program_name] ... -- Note: This is unreliable on different terminals");
        println!("Enter 'exit' to exit");
        println!();
        loop
        {
            struct Exit;

            let res = panic::catch_unwind(||
            {
                let input = get_input("Enter expression: ");
                if input == "exit" { panic::panic_any(Exit{}) }
                let tokens =  lexer::extract_tokens(input.as_str());
                println!("{}", parser::solve(&tokens));
            });
            if let Err(panic) = res 
            {
                if let Some(e) = panic.downcast_ref::<&str>() { println!("{}", e) }
                else if let Some(e) = panic.downcast_ref::<String>() { println!("{}", e) }
                else if let Some(_) = panic.downcast_ref::<Exit>() { return }
                else { panic::resume_unwind(panic) }
            }
        }
    }
}