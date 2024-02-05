use super::file_to_lexer::Token;

#[derive(Debug)]
enum JsonNode {
    Object(Vec<(String, JsonNode)>),
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<JsonNode>),
}

#[allow(dead_code, unused_variables)]
pub fn parse(tokens: Vec<Token>) {
    let mut context = Vec::new();
    let mut root = JsonNode::Object(Vec::new());
    let mut in_array = false;

    for token in tokens {
        match token {
            Token::String(string) => {
                parse_string(string, &mut context);
            }
            Token::LBrace => {
                parse_lbrace(&mut context);
            }
            Token::RBrace => {
                parse_rbrace(&mut context, &mut root);
            }
            Token::LBracket => {
                parse_lbracket(&mut context, &mut in_array);
            }
            Token::RBracket => {
                parse_rbracket(&mut context, &mut root, &mut in_array);
            }
            Token::Assign => {
                // do nothing
            }
            Token::Comma => {
                // do nothing
            }
            Token::NewLine => {
                // do nothing
            }
            Token::EOF => {
                // do nothing
            }
            Token::Boolean(boolean) => {
                parse_boolean(boolean, &mut context);
            }
            Token::Number(value) => {
                if in_array {
                    if let Some(JsonNode::Array(arr)) = context.last_mut() {
                        arr.push(JsonNode::Number(value as f64));
                    }
                } else {
                    parse_number(value as f64, &mut context);
                }
            }
        }
    }

    println!("{:?}", root);
}

#[allow(dead_code)]
fn parse_string(value: String, context: &mut Vec<JsonNode>) {
    if let Some(JsonNode::Object(obj)) = context.last_mut() {
        obj.push(("Identifier".to_string(), JsonNode::String(value)))
    }
}

fn parse_lbrace(context: &mut Vec<JsonNode>) {
    context.push(JsonNode::Object(Vec::new()));
}

fn parse_rbrace(context: &mut Vec<JsonNode>, root: &mut JsonNode) {
    if let Some(JsonNode::Object(obj)) = context.pop() {
        if let Some(JsonNode::Object(ref mut parent)) = context.last_mut() {
            parent.push(("name".to_string(), JsonNode::Object(obj)))
        } else {
            *root = JsonNode::Object(obj);
        }
    }
}

fn parse_lbracket(context: &mut Vec<JsonNode>, in_array: &mut bool) {
    context.push(JsonNode::Array(Vec::new()));
    *in_array = true;
}

fn parse_rbracket(context: &mut Vec<JsonNode>, root: &mut JsonNode, in_array: &mut bool) {
    if let Some(JsonNode::Array(arr)) = context.pop() {
        if let Some(JsonNode::Object(ref mut parent)) = context.last_mut() {
            parent.push(("Value".to_string(), JsonNode::Array(arr)))
        } else {
            *root = JsonNode::Array(arr);
        }
        *in_array = false;
    }
}

fn parse_boolean(value: bool, context: &mut Vec<JsonNode>) {
    if let Some(JsonNode::Object(obj)) = context.last_mut() {
        obj.push(("value".to_string(), JsonNode::Bool(value)))
    }
}

fn parse_number(value: f64, context: &mut Vec<JsonNode>) {
    if let Some(JsonNode::Object(obj)) = context.last_mut() {
        obj.push(("".to_string(), JsonNode::Number(value)))
    }
}
