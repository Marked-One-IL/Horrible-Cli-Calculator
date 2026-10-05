pub enum Token<'expr> 
{
    Symbol(u8),
    Num(f64),
    Name(&'expr str)
}

pub fn extract_tokens<'expr>(expr: &'expr str) -> Vec<Token<'expr>>
{
    let mut tokens: Vec<Token<'expr>> = Vec::new();
    let mut curr_expr = expr;

    while !curr_expr.is_empty() 
    {
        curr_expr = curr_expr.trim_ascii_start();
        if curr_expr.is_empty() { break }

        if let Some(symbol) = extract_symbol(&mut curr_expr) { tokens.push(Token::Symbol(symbol)) }
        else if let Some(num) = extract_num(&mut curr_expr) { tokens.push(Token::Num(num)) }
        else if let Some(name) = extract_name(&mut curr_expr) { tokens.push(Token::Name(name)) }
        else { panic!("Invalid character") }
    }

    tokens
}

fn extract_symbol(expr: &mut &str) -> Option<u8> 
{
    let c = *expr.as_bytes().first().unwrap();
    if [b'+', b'-', b'*', b'/', b'%', b'(', b')', b','].contains(&c) 
    {
        *expr = &expr[1..];
        return Some(c)
    }

    None
}
fn extract_num(expr: &mut &str) -> Option<f64> 
{
    if let Ok((val, n)) = lexical_core::parse_partial::<f64>(expr.as_bytes()) 
    {
        *expr = &expr[n..];
        if !expr.is_empty() && expr.as_bytes().first().copied().unwrap().is_ascii_alphanumeric() { panic!("Invalid number") }
        return Some(val)
    }

    None
}
fn extract_name<'expr>(expr: &mut &'expr str) -> Option<&'expr str> 
{
    let first = expr.as_bytes().first().copied().unwrap();
    if !first.is_ascii_alphabetic() && first != b'_' { return None }

    let t_expr = &expr[1..];
    for (i, c) in t_expr.as_bytes().iter().enumerate() {
        if !c.is_ascii_alphanumeric() && *c != b'_' 
        {
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