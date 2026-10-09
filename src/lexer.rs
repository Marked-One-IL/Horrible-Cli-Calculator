use core::f64;
use console::style;

#[derive(PartialEq, Clone, Copy)]
pub enum Symbol {
    Low,      // <
    LowEq,    // <=
    Eq,       // ==
    Neq,      // !=
    HighEq,   // >=
    High,     // >
    Ex,       // !
    Plus,     // +
    Min,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    ParStart, // (
    ParEnd,   // )
    Comma     // ,
}
pub enum TokenContent<'expr>
{
    Symbol(Symbol),
    Num(f64),
    Name(&'expr str)
}
pub struct Token<'expr> {
    pub content: TokenContent<'expr>,
    pub pos: usize
}
pub struct Error
{
    msg: &'static str,
    pos: usize
}

impl<'expr> Token<'expr> {
    fn new(content: TokenContent<'expr>, pos: usize) -> Self {
        Self { content, pos }
    }
}
impl Error {
    pub fn new(msg: &'static str, pos: usize) -> Self {
        Self { msg, pos }
    }

    pub fn print(&self, full_expr: &str) 
    {
        println!("{}", full_expr);
        for _ in 0..self.pos { print!("{}", style("-").yellow()) }
        println!("{}", style("^").yellow());
        println!("{}", style(self.msg).red());
    }
}

pub fn extract_tokens<'expr>(expr: &'expr str) -> Result<Vec<Token<'expr>>, Error>
{
    let mut tokens: Vec<Token<'expr>> = Vec::new();
    let mut curr_expr = expr;

    while !curr_expr.is_empty() 
    {
        curr_expr = curr_expr.trim_ascii_start();
        if curr_expr.is_empty() { break }

        let pos = expr.len() - curr_expr.len();
        if let Some(symbol) = extract_symbol(&mut curr_expr) { tokens.push( Token::new(TokenContent::Symbol(symbol), pos ) ) }
        else if let Some(num) = extract_num(pos, &mut curr_expr)? { tokens.push( Token::new(TokenContent::Num(num), pos) ) }
        else if let Some(len) = extract_str_len(pos, &mut curr_expr)? { tokens.push( Token::new(TokenContent::Num(len), pos) ) }
        else if let Some(name) = extract_name(&mut curr_expr) 
        {
            if let Some(constant) = extract_const_from_name(name) { tokens.push( Token::new(TokenContent::Num(constant), pos) ) } 
            else { tokens.push( Token::new(TokenContent::Name(name), pos) ) }
        }
        else { return Err( Error::new("Invalid character", pos) ) }
    }

    Ok(tokens)
}

fn extract_symbol(expr: &mut &str) -> Option<Symbol> 
{
    // Must be separated because if we scan for example '<' it would not detect '<='.
    static SYMBOLS_2: [(&'static str, Symbol); 4] =
    [("<=", Symbol::LowEq), ("==", Symbol::Eq), ("!=", Symbol::Neq), (">=", Symbol::HighEq)];

    static SYMBOLS_1: [(&'static str, Symbol); 11] =
    [("<", Symbol::Low), (">", Symbol::High),  ("!", Symbol::Ex),
    ("+", Symbol::Plus), ("-", Symbol::Min), ("*", Symbol::Mul), ("/", Symbol::Div), ("%", Symbol::Mod),
    ("(", Symbol::ParStart), (")", Symbol::ParEnd), (",", Symbol::Comma)];

    for symbol in SYMBOLS_2 {
        if expr.starts_with(symbol.0) {
            *expr = &expr[symbol.0.len()..];
            return Some(symbol.1)
        }
    }
    for symbol in SYMBOLS_1 {
        if expr.starts_with(symbol.0) {
            *expr = &expr[symbol.0.len()..];
            return Some(symbol.1)
        }
    }

    None
}
fn extract_name<'expr>(expr: &mut &'expr str) -> Option<&'expr str> 
{
    if !expr.as_bytes()[0].is_ascii_alphabetic() { return None }

    let t_expr = &expr[1..];
    for (i, c) in t_expr.as_bytes().iter().enumerate() {
        if !c.is_ascii_alphanumeric() {
            let n = i + 1;
            let t = &expr[..n];
            *expr = &expr[n..];
            return Some(t)
        }
    }

    let t = *expr;
    *expr = "";
    Some(t)
}
fn extract_num(pos: usize, expr: &mut &str) -> Result<Option<f64>, Error>
{
    if let Ok((val, n)) = lexical_core::parse_partial::<f64>(expr.as_bytes()) {
        *expr = &expr[n..];
        if !expr.is_empty() && expr.as_bytes().first().copied().unwrap().is_ascii_alphanumeric() { return Err( Error::new("Invalid number", pos) ) }
        return Ok(Some(val))
    }
    Ok(None)
}
fn extract_str_len(pos: usize, expr: &mut &str) -> Result<Option<f64>, Error>
{
    let symbol; 
    if expr.starts_with("'") { symbol = "'"; }
    else if expr.starts_with("\"") { symbol = "\""; }
    else { return Ok(None) }

    let new_expr = &expr[1..];
    let end = new_expr.find(symbol).ok_or( Error::new("Could not find the string literal end", pos) )?;
    *expr = &new_expr[(end+1)..];
    Ok(Some(end as f64))
}
fn extract_const_from_name(name: &str) -> Option<f64>
{
    match name {
        "pi"  => Some(f64::consts::PI),
        "e"   => Some(f64::consts::E),
        "phi" => Some(f64::consts::GOLDEN_RATIO),
        _ => None
    }
}