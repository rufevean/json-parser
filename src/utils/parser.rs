use super::file_to_lexer::Token;
#[allow(dead_code,unused_variables)]
pub fn parse(tokens: Vec<Token>) {
    for token in tokens {
        match token {
            Token::String(ref String) => {
                println!("String");
                println!("{:?}",token);
                parse_string(token);
            }
            Token::LBrace => {
                parse_lbrace(token);
            }
            Token::RBrace => {
                parse_rbrace(token);
            }
            Token::LBracket => {
                parse_lbracket(token);
            }
            Token::RBracket => {
                parse_rbracket(token);
            }
            Token::Assign => {
                parse_assign(token);
            }
            Token::Comma => {
                parse_comma(token);
            }
            Token::NewLine => {
                parse_newline(token);
            }
            Token::EOF => {
                parse_eof(token);
            }
            Token::Boolean(boolean) => {
                parse_boolean(token);
            }
            Token::Number(usize) => {
                parse_number(token);
            }
        }
    }
}

#[allow(dead_code)]
fn parse_string(token: Token) {
    println!("String");
}

fn parse_lbrace(token: Token) {
    println!("LBrace");
}

fn parse_rbrace(token: Token) {
    println!("RBrace");
}
fn parse_lbracket(token: Token) {
    println!("LBracket");
}
fn parse_rbracket(token: Token) {
    println!("RBracket");
}
fn parse_assign(token: Token) {
    println!("Assign");
}
fn parse_comma(token: Token) {
    println!("Comma");
}
fn parse_newline(token: Token) {
    println!("NewLine");
}
fn parse_eof(token: Token) {
    println!("EOF");
}
fn parse_boolean(token: Token) {
    println!("Boolean");
}
fn parse_number(token: Token) {
    println!("Number");
}
