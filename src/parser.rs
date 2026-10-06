use crate::lexer::{self, Symbol, Token, TokenContent};
use core::f64;

enum Func {
    Sqrt(Box<Ast>),          // sqrt(x)
    Sin(Box<Ast>),           // sin(x)
    Cos(Box<Ast>),           // cos(x)
    Tan(Box<Ast>),           // tan(x)
    Pow(Box<Ast>, Box<Ast>), // pow(x, e)
    Log(Box<Ast>, Box<Ast>)  // log(x, p)
}
enum AstContent {
    Bin(Box<Ast>, Symbol, Box<Ast>), // () symbol ()
    Unary(Symbol, Box<Ast>),         // symbol()
    Call(Func),                      // foo(x, ...)
    Val(f64)                         // num
}
struct Ast {
    pub content: AstContent,
    pub pos: usize
}
pub struct Error 
{
    msg: &'static str,
    pos: usize
}

impl Ast {
    fn new(content: AstContent, pos: usize) -> Self {
        Self { content, pos }
    }
}

impl Error {
    fn new(msg: &'static str, pos: usize) -> Self {
        Self { msg, pos }
    }
    
    pub fn print(&self, full_expr: &str, tokens: &Vec<Token>) 
    {
        lexer::Error::new(self.msg, tokens[self.pos].pos).print(full_expr)
    }
}

pub fn solve(tokens: &Vec<Token>) -> Result<f64, Error>
{
    if tokens.is_empty() { return Err( Error::new("A binary expression must have a right expression", 0)) }
    solve_helper(&build_all(tokens, &mut 0, tokens.len() - 1, false)?)
}

fn solve_helper(ast: &Box<Ast>) -> Result<f64, Error>
{
    match &ast.content {
        AstContent::Bin(left, opr, right) => 
        {
            let left_v = solve_helper(&left)?;
            let right_v = solve_helper(&right)?;

            match opr {
                Symbol::Plus => Ok(left_v + right_v),
                Symbol::Min  => Ok(left_v - right_v),
                Symbol::Mul  => Ok(left_v * right_v),
                Symbol::Div  => Ok(left_v / right_v),
                Symbol::Mod  => Ok(left_v % right_v),
                _ => unreachable!()
            }
        }
        AstContent::Unary(opr, expr) =>
        {
            let expr_v = solve_helper(&expr)?;

            match opr {
                Symbol::Plus => Ok(expr_v),
                Symbol::Min  => Ok(-expr_v),
                _ => unreachable!()
            }
        }
        AstContent::Call(func) => 
        {
            match func {
                Func::Sqrt(param) => 
                {
                    let param_v = solve_helper(&param)?;
                    if param_v < 0.0 { return Err( Error::new("For sqrt(x): x >= 0.0", param.pos) ) }
                    Ok(f64::sqrt(param_v))
                },
                Func::Sin(param) => 
                {
                    let param_v = solve_helper(&param)?;
                    Ok(f64::sin(param_v.to_radians()))
                },
                Func::Cos(param) => 
                {
                    let param_v = solve_helper(&param)?;
                    Ok(f64::cos(param_v.to_radians()))
                },
                Func::Tan(param) => 
                {
                    let param_v = solve_helper(&param)?;
                    Ok(f64::tan(param_v.to_radians()))
                },
                Func::Pow(param1, param2) => 
                {
                    let param1_v = solve_helper(&param1)?;
                    let param2_v = solve_helper(&param2)?;
                    Ok(f64::powf(param1_v, param2_v))
                }
                Func::Log(param1, param2) => 
                {
                    let param1_v = solve_helper(&param1)?;
                    let param2_v = solve_helper(&param2)?;
                    if param1_v < 0.0 || param1_v == 1.0 { return Err( Error::new("For log(x, ...): x > 0.0 and x != 1.0", param1.pos) ) }
                    if param2_v <= 0.0 { return Err( Error::new("For log(..., x): x > 0.0", param2.pos) ) }

                    Ok(f64::log10(param2_v) / f64::log10(param1_v))
                }
            }
        }
        AstContent::Val(val) => Ok(*val)
    }
}

fn build_all(tokens: &Vec<Token>, start: &mut usize, end: usize, was_super: bool) -> Result<Box<Ast>, Error>
{
    let mut left = build_single(tokens, start, end)?;

    while *start <= end 
    {
        let TokenContent::Symbol(symbol) = tokens[*start].content else { return Err( Error::new("A binary expression must have a symbol operation", *start) ) };

        let is_curr_super = match symbol {
            Symbol::Plus | Symbol::Min => Ok(false),
            Symbol::Mul  | Symbol::Div | Symbol::Mod => Ok(true),
            _ => Err( Error::new("Invalid binary operation symbol", *start) ) }?;

        if !is_curr_super && was_super { break }
        *start += 1;
        if *start > end { return Err( Error::new("A binary expression must have a right expression", *start) ) }

        left = Box::new(Ast::new(AstContent::Bin(left, symbol, build_all(tokens, start, end, is_curr_super)?), *start));
    }

    Ok(left)
}
fn build_single(tokens: &Vec<Token>, start: &mut usize, end: usize) -> Result<Box<Ast>, Error>
{
    if let Some(unary) = build_unary(tokens, start, end)? { return Ok(unary) }
    else if let Some(sub) = build_sub(tokens, start, end)? { return Ok(sub) }
    else if let Some(call) = build_call(tokens, start, end)? { return Ok(call) }
    else if let Some(val) = build_val(tokens, start)? { return Ok(val) }

    Err( Error::new("Invalid expression structure", *start) )
}
fn build_unary(tokens: &Vec<Token>, start: &mut usize, end: usize) -> Result<Option<Box<Ast>>, Error>
{
    let TokenContent::Symbol(symbol) = tokens[*start].content else { return Ok(None) }; 
    if symbol != Symbol::Plus && symbol != Symbol::Min { return Ok(None) }
    *start += 1;
    if *start > end { return Err( Error::new("An unary expression must have an actual expression", *start) ) }

    Ok(Some(Box::new(Ast::new(AstContent::Unary(symbol, build_single(tokens, start, end)?), *start))))
}
fn build_sub(tokens: &Vec<Token>, start: &mut usize, end: usize) -> Result<Option<Box<Ast>>, Error>
{
    let TokenContent::Symbol(first_symbol) = tokens[*start].content else { return Ok(None) };
    if first_symbol != Symbol::ParStart { return Ok(None) }   
    *start += 1;

    Ok(Some(build_sub_general(tokens, start, end, "A sub expression must end with a ')'", "A sub expression must have an actual expression")?))
}
fn build_call(tokens: &Vec<Token>, start: &mut usize, end: usize) -> Result<Option<Box<Ast>>, Error>
{
    let TokenContent::Name(name) = tokens[*start].content else { return Ok(None) };
    *start += 1;

    if let TokenContent::Symbol(first_symbol) = tokens.get(*start).ok_or(Error::new("A function call expression after the function name must have a symbol", *start))?.content {
        *start += 1;
        if first_symbol != Symbol::ParStart { return Err( Error::new("A function call expression after the function name must have a symbol of '('", *start) ) }
    }

    match name {
        "sqrt" => { Ok(Some(Box::new(Ast::new(AstContent::Call(Func::Sqrt( build_1_params(tokens, start, end)? )), *start)))) }
        "sin" => { Ok(Some(Box::new(Ast::new(AstContent::Call(Func::Sin( build_1_params(tokens, start, end)? )), *start)))) }
        "cos" => { Ok(Some(Box::new(Ast::new(AstContent::Call(Func::Cos( build_1_params(tokens, start, end)? )), *start)))) }
        "tan" => { Ok(Some(Box::new(Ast::new(AstContent::Call(Func::Tan( build_1_params(tokens, start, end)? )), *start)))) }
        "pow" => {
            let (param1, param2) = build_2_params(tokens, start, end)?;
            Ok(Some(Box::new(Ast::new(AstContent::Call(Func::Pow(param1, param2)), *start))))
        }
        "log" => {
            let (param1, param2) = build_2_params(tokens, start, end)?;
            Ok(Some(Box::new(Ast::new(AstContent::Call(Func::Log(param1, param2)), *start))))
        }
        _ => { Err( Error::new("Invalid function name", *start) ) }
    }
}
fn build_val(tokens: &Vec<Token>, start: &mut usize) -> Result<Option<Box<Ast>>, Error>
{
    let TokenContent::Num(num) = tokens[*start].content else { return Ok(None) };
    *start += 1;
    Ok(Some(Box::new(Ast::new(AstContent::Val(num), *start))))
}

fn build_sub_general(tokens: &Vec<Token>, start: &mut usize, end: usize, no_end_err: &'static str, empty_err: &'static str) -> Result<Box<Ast>, Error>
{
    let mut level: usize = 0;
    let mut end_par: Option<usize> = None;

    for i in *start..(end+1) {
        let TokenContent::Symbol(symbol) = tokens[i].content else { continue };
        if symbol == Symbol::ParStart { level += 1 }
        else if symbol == Symbol::ParEnd {
            if level == 0 {
                end_par = Some(i);
                break;
            }
            level -= 1;
        }
    }
    let end_par_v = end_par.ok_or( Error::new(no_end_err, *start) )?;
    let mut t_start = *start;
    let t_end = end_par_v - 1;
    *start = end_par_v + 1;
    if t_start > t_end { return Err( Error::new(empty_err, *start) ) }

    build_all(tokens, &mut t_start, t_end, false)
}
fn build_1_params(tokens: &Vec<Token>, start: &mut usize, end: usize) -> Result<Box<Ast>, Error>
{
    build_sub_general(tokens, start, end, "A function call expression must end with a ')'", "A function call expression must have a parameter expression")
}
fn build_2_params(tokens: &Vec<Token>, start: &mut usize, end: usize) -> Result<(Box<Ast>, Box<Ast>), Error>
{
    let mut level: usize = 0;
    let mut end_par: Option<usize> = None;
    let mut comma: Option<usize> = None;

    for i in *start..(end+1) 
    {
        let TokenContent::Symbol(symbol) = tokens[i].content else { continue };
        if symbol == Symbol::ParStart { level += 1 }
        else if symbol == Symbol::ParEnd {
            if level == 0 {
                end_par = Some(i);
                break;
            }
            level -= 1;
        }
        if symbol == Symbol::Comma && level == 0 && comma.is_none() { 
            comma = Some(i) 
        }
    }
    let comma_v = comma.ok_or( Error::new("Could not find the ',' seperator for the function call", *start) )?;
    let mut t_start1 = *start;
    let t_end1 = comma_v - 1;
    *start = comma_v + 1;
    if t_start1 > t_end1 { return Err( Error::new("A function call expression first parameter must have an expression", *start) ) }

    let end_par_v = end_par.ok_or( Error::new("A function call expression must end with a ')'", *start) )?;
    let mut t_start2 = *start;
    let t_end2 = end_par_v - 1;
    *start = end_par_v + 1;
    if t_start2 > t_end2 { return Err( Error::new("A function call expression second parameter must have an expression", *start) ) }

    let param1_expr = build_all(tokens, &mut t_start1, t_end1, false)?;
    let param2_expr = build_all(tokens, &mut t_start2, t_end2, false)?;

    Ok((param1_expr, param2_expr))
}