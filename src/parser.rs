use core::f64;
use crate::lexer::Token;

enum Func 
{
    Sqrt(Box<Ast>),          // sqrt(x)
    Sin(Box<Ast>),           // sin(x)
    Cos(Box<Ast>),           // cos(x)
    Tan(Box<Ast>),           // tan(x)
    Pow(Box<Ast>, Box<Ast>), // pow(x, e)
    Log(Box<Ast>, Box<Ast>)  // log(x, p)
}

enum Ast 
{
    Bin(Box<Ast>, u8, Box<Ast>),   // () symbol ()
    Unary(u8, Box<Ast>),           // symbol()
    Call(Func),                    // foo(x, ...)
    Val(f64)                       // num
}

pub fn solve(tokens: &Vec<Token>) -> f64
{
    solve_helper(build_all(tokens, &mut 0, tokens.len() - 1, false, "There is no expression"))
}

fn solve_helper(ast: Box<Ast>) -> f64
{
    match *ast {
        Ast::Bin(left, opr, right) => 
        {
            let left_v = solve_helper(left);
            let right_v = solve_helper(right);

            match opr
            {
                b'+' => left_v + right_v,
                b'-' => left_v - right_v,
                b'*' => left_v * right_v,
                b'/' => left_v / right_v,
                b'%' => left_v % right_v,
                _ => unreachable!()
            }
        }
        Ast::Unary(opr, expr) =>
        {
            let expr_v = solve_helper(expr);

            match opr
            {
                b'+' => expr_v,
                b'-' => -expr_v,
                _ => unreachable!()
            }
        }
        Ast::Call(func ) => {      
            match func {
                Func::Sqrt(param) => 
                {
                    let param_v = solve_helper(param);
                    if param_v < 0.0 { panic!("Cannot pass a negative parameter expression to sqrt"); }
                    f64::sqrt(param_v)
                },
                Func::Sin(param) => 
                {
                    let param_v = solve_helper(param);
                    f64::sin(param_v  * (f64::consts::PI / 180.0))
                },
                Func::Cos(param) => 
                {
                    let param_v = solve_helper(param);
                    f64::cos(param_v * (f64::consts::PI / 180.0))
                },
                Func::Tan(param) => 
                {
                    let param_v = solve_helper(param);
                    f64::tan(param_v * (f64::consts::PI / 180.0))
                },
                Func::Pow(param1, param2) => 
                {
                    let param1_v = solve_helper(param1);
                    let param2_v = solve_helper(param2);
                    f64::powf(param1_v, param2_v)
                }
                Func::Log(param1, param2) => 
                {
                    let param1_v = solve_helper(param1);
                    let param2_v = solve_helper(param2);
                    if param1_v < 0.0 || param1_v == 1.0 { panic!("For log(x, ...): x > 0.0 and x != 1.0"); }
                    if param2_v <= 0.0 { panic!("For log(..., x): x <= 0.0"); }

                    f64::log10(param2_v) / f64::log10(param1_v)
                }
            }
        }
        Ast::Val(val) => val
    }
}

fn is_bin_opr_superior(symbol: u8) -> bool
{
    match symbol {
        b'+' | b'-' => false,
        b'*' | b'/' | b'%' => true,
        _ => panic!("Invalid binary operation symbol")
    }
}
fn build_all(tokens: &Vec<Token>, start: &mut usize, end: usize, was_super: bool, outbound_msg: &'static str) -> Box<Ast>
{
    let mut left = build_single(tokens, start, end, outbound_msg);

    while *start <= end 
    {
        let Token::Symbol(symbol) = tokens[*start] else { panic!("A binary expression must have a symbol operation") };
        let is_curr_super = is_bin_opr_superior(symbol);

        if !is_curr_super && was_super { break }
        *start += 1;
        left = Box::new(Ast::Bin(left, symbol, build_all(tokens, start, end, is_curr_super,
            "A binary expression must have a right expression")));
    }

    left
}
fn build_single(tokens: &Vec<Token>, start: &mut usize, end: usize, outbound_msg: &'static str) -> Box<Ast>
{
    if let Some(unary) = build_unary(tokens, start, end, outbound_msg) { return unary }
    else if let Some(sub) = build_sub(tokens, start, end, outbound_msg) { return sub }
    else if let Some(call) = build_call(tokens, start, end, outbound_msg) { return call }
    else if let Some(val) = build_val(tokens, start, end, outbound_msg) { return val }

    panic!("Invalid expression structure")
}
fn build_unary(tokens: &Vec<Token>, start: &mut usize, end: usize, outbound_msg: &'static str) -> Option<Box<Ast>>
{
    if *start > end { std::panic::panic_any(outbound_msg) }

    let Token::Symbol(symbol) = tokens[*start] else { return None }; 
    if symbol != b'+' && symbol != b'-' { return None }
    *start += 1;

    Some(Box::new(Ast::Unary(symbol, build_single(tokens, start, end, "An unary expression must have an actual expression")))) 
}
fn build_sub(tokens: &Vec<Token>, start: &mut usize, end: usize, outbound_msg: &'static str) -> Option<Box<Ast>>
{
    if *start > end { std::panic::panic_any(outbound_msg) }

    let Token::Symbol(first_symbol) = tokens[*start] else { return None };
    if first_symbol != b'(' { return None }   
    *start += 1;

    let mut level: usize = 0;
    let mut end_par: Option<usize> = None;
    for i in *start..(end+1) {
        let Token::Symbol(symbol) = tokens[i] else { continue };
        if symbol == b'(' { level += 1 }
        else if symbol == b')' {
            if level == 0 
            {
                end_par = Some(i);
                break;
            }
            level -= 1;
        }
    }

    let end_par_v = end_par.expect("A sub expression must end with a ')'");
    let mut t_start = *start;
    let t_end = end_par_v - 1;
    *start = end_par_v + 1;
    Some(build_all(tokens, &mut t_start, t_end, false, "A sub expression must have an actual expression"))
}
fn build_call(tokens: &Vec<Token>, start: &mut usize, end: usize, outbound_msg: &'static str) -> Option<Box<Ast>>
{
    if *start > end { std::panic::panic_any(outbound_msg) }

    let Token::Name(name) = tokens[*start] else { return None };
    *start += 1;

    if let Token::Symbol(first_symbol) =
        *tokens.get(*start).expect("A function call expression after the function name must have a symbol")
    {
        *start += 1;
        if first_symbol != b'(' { panic!("A function call expression after the function name must have a symbol of '('") }
    }

    let is_sqrt = name == "sqrt";
    let is_pow = name == "pow";
    let is_log = name == "log";
    let is_sin = name == "sin";
    let is_cos = name == "cos";
    let is_tan = name == "tan";
    let is_1_params = is_sqrt || is_sin || is_cos || is_tan;
    let is_2_params = is_pow || is_log;

    if is_1_params
    {
        let mut level: usize = 0;
        let mut end_par: Option<usize> = None;

        for i in *start..(end+1) {
            let Token::Symbol(symbol) = tokens[i] else { continue };
            if symbol == b'(' { level += 1 }
            else if symbol == b')' {
                if level == 0 
                {
                    end_par = Some(i);
                    break;
                }
                level -= 1;
            }
        }
        let end_par_v = end_par.expect("A function call expression must end with a ')'");
        let mut t_start = *start;
        let t_end = end_par_v - 1;
        *start = end_par_v + 1;

        let param_expr = build_all(tokens, &mut t_start, t_end, false, 
            "A function call expression must have a parameter expression");
        if is_sqrt { return Some(Box::new(Ast::Call(Func::Sqrt(param_expr)))) }
        else if is_sin { return Some(Box::new(Ast::Call(Func::Sin(param_expr)))) }
        else if is_cos { return Some(Box::new(Ast::Call(Func::Cos(param_expr)))) }
        else /* is_tan */ { return Some(Box::new(Ast::Call(Func::Tan(param_expr)))) }
    }
    else if is_2_params
    {
        let mut level: usize = 0;
        let mut end_par: Option<usize> = None;
        let mut comma: Option<usize> = None;

        for i in *start..(end+1) 
        {
            let Token::Symbol(symbol) = tokens[i] else { continue };
            if symbol == b'(' { level += 1 }
            else if symbol == b')' {
                if level == 0
                {
                    end_par = Some(i);
                    break;
                }
                level -= 1;
            }
            if symbol == b',' && level == 0 && comma.is_none() { 
                comma = Some(i) 
            }
        }
        let comma_v = comma.expect("Could not find the ',' seperator for the function call");
        let mut t_start1 = *start;
        let t_end1 = comma_v - 1;
        *start = comma_v + 1;

        let end_par_v = end_par.expect("A function call expression must end with a ')'");
        let mut t_start2 = *start;
        let t_end2 = end_par_v - 1;
        *start = end_par_v + 1;

        let param1_expr = build_all(tokens, &mut t_start1, t_end1, false,
            "A function call expression first parameter must have an expression");
        let param2_expr = build_all(tokens, &mut t_start2, t_end2, false,
            "A function call expression second parameter must have an expression");
        if is_pow { return Some(Box::new(Ast::Call(Func::Pow(param1_expr, param2_expr)))) } 
        else /* is_log */ { return Some(Box::new(Ast::Call(Func::Log(param1_expr, param2_expr)))) }
    }
    else { panic!("Invalid function name") }
}
fn build_val(tokens: &Vec<Token>, start: &mut usize, end: usize, outbound_msg: &'static str) -> Option<Box<Ast>>
{
    if *start > end { std::panic::panic_any(outbound_msg) }

    let Token::Num(num) = tokens[*start] else { return None };
    *start += 1;
    Some(Box::new(Ast::Val(num)))
}