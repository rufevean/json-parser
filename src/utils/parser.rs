#[derive(Debug)]
pub enum Token {
    ObjectStart,
    ObjectEnd,
    String(String),
    Colon,
    Comma,
}
pub fn parse(tokens: &[Token]) -> Result<(), &'static str> {
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::ObjectStart => {
                match iter.next() {
                    Some(Token::String(key)) => {
                        match iter.next() {
                            Some(Token::Colon) => {
                                match iter.next() {
                                    Some(Token::String(value)) => {
                                        println!("Key: {}, Value: {}", key, value);
                                    }
                                    _ => return Err("Expected a string value after colon."),
                                }
                            }
                            _ => return Err("Expected a colon after key."),
                        }
                    }
                    _ => return Err("Expected a string key."),
                }

                match iter.peek() {
                    Some(Token::Comma) => {
                        iter.next(); 
                    }
                    Some(Token::ObjectEnd) => {
                        iter.next(); 
                        break; 
                    }
                    _ => return Err("Expected a comma or object end after key-value pair."),
                }
            }
            _ => return Err("Expected an object start."),
        }
    }

    Ok(())
}
