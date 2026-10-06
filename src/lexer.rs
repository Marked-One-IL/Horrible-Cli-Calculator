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
pub enum Token<'expr> {
    Symbol(Symbol),
    Num(f64),
    Name(&'expr str)
}

pub fn extract_tokens<'expr>(expr: &'expr str) -> Result<Vec<Token<'expr>>, &'static str>
{
    let mut tokens: Vec<Token<'expr>> = Vec::new();
    let mut curr_expr = expr;

    while !curr_expr.is_empty() 
    {
        curr_expr = curr_expr.trim_ascii_start();
        if curr_expr.is_empty() { break }

        if let Some(symbol) = extract_symbol(&mut curr_expr) { tokens.push(Token::Symbol(symbol)) }
        else if let Some(num) = extract_num(&mut curr_expr)? { tokens.push(Token::Num(num)) }
        else if let Some(name) = extract_name(&mut curr_expr) { tokens.push(Token::Name(name)) }
        else { return Err("Invalid character") }
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
    if !first.is_ascii_alphabetic() && first != b'_' { return None }

    let t_expr = &expr[1..];
    for (i, c) in t_expr.as_bytes().iter().enumerate() {
        if !c.is_ascii_alphanumeric() && *c != b'_' {
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
fn extract_num(expr: &mut &str) -> Result<Option<f64>, &'static str>
{
    if let Ok((val, n)) = lexical_core::parse_partial::<f64>(expr.as_bytes()) {
        *expr = &expr[n..];
        if !expr.is_empty() && expr.as_bytes().first().copied().unwrap().is_ascii_alphanumeric() { return Err("Invalid number") }
        return Ok(Some(val))
    }
    Ok(None)
}