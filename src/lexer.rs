#[derive(PartialEq, Clone, Copy)]
pub enum Symbol {
    Plus,
    Min,
    Mul,
    Div,
    Mod,
    ParStart,
    ParEnd,
    Comma
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
        for _ in 0..self.pos { print!("-") }
        println!("^");
        println!("{}", self.msg);
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
        else if let Some(name) = extract_name(&mut curr_expr) { tokens.push( Token::new(TokenContent::Name(name), pos) ) }
        else { return Err( Error::new("Invalid character", pos) ) }
    }

    Ok(tokens)
}

fn extract_symbol(expr: &mut &str) -> Option<Symbol> 
{
    let symbol = match *expr.as_bytes().first().unwrap() {
        b'+' => Some(Symbol::Plus),
        b'-' => Some(Symbol::Min), 
        b'*' => Some(Symbol::Mul),
        b'/' => Some(Symbol::Div),
        b'%' => Some(Symbol::Mod),
        b'(' => Some(Symbol::ParStart),
        b')' => Some(Symbol::ParEnd),
        b',' => Some(Symbol::Comma),
        _ => return None
    };

    *expr = &expr[1..];
    symbol
}
fn extract_name<'expr>(expr: &mut &'expr str) -> Option<&'expr str> 
{
    let first = expr.as_bytes().first().copied().unwrap();
    if !first.is_ascii_alphabetic() { return None }

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